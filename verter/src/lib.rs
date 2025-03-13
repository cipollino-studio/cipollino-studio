
#[cfg(test)]
mod test;

use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    InvalidFile,
    InvalidPointer,
    DeletedPointer,
    CorruptedFile
}

const BYTES_IN_U64: u64 = 8;

#[derive(Clone, Copy)]
pub struct Config {
    /// The magic bytes at the start of the file
    pub magic_bytes: &'static [u8],
    /// The number of bytes per page, excluding the page header
    pub page_size: usize
}

impl Default for Config {

    fn default() -> Self {
        Self {
            magic_bytes: b"VERTER__",
            page_size: 120
        }
    }

}

#[derive(Clone, Copy)]
enum PageHeader {
    /// There is a next page.
    /// u64 -> The pointer of the next page
    NextPage(u64),
    /// This is the last page.
    /// u64 -> The number of bytes in this page
    FinalPage(u64),
    /// This is a deleted page.
    /// u64 -> Pointer to the next deleted page, or 0 if there are no more deleted pages.
    DeletedPage(u64)
}

impl PageHeader {

    const FLAG_MASK: u64 = 3u64 << 62;
    const NEXT_PAGE_FLAG: u64 = 0u64 << 62;
    const FINAL_PAGE_FLAG: u64 = 1u64 << 62;
    const DELETED_PAGE_FLAG: u64 = 2u64 << 62; 

    fn to_u64(self) -> u64 {
        match self {
            PageHeader::NextPage(next) => Self::NEXT_PAGE_FLAG | next,
            PageHeader::FinalPage(size) => Self::FINAL_PAGE_FLAG | size,
            PageHeader::DeletedPage(next) => Self::DELETED_PAGE_FLAG | next
        }
    }

    fn from_u64(val: u64) -> Self {
        let subval = val & !Self::FLAG_MASK; 
        match val & Self::FLAG_MASK {
            Self::NEXT_PAGE_FLAG => Self::NextPage(subval),
            Self::FINAL_PAGE_FLAG => Self::FinalPage(subval),
            Self::DELETED_PAGE_FLAG | _ => Self::DeletedPage(subval),
        }
    }

}

pub struct File {
    file: std::fs::File,
    config: Config
}

impl File {

    /// Open a file.
    /// Creates and initiates it if it currently does not exist.
    /// Will return an error if the file is invalid(ie has incorrect magic bytes).
    pub fn open<P: AsRef<std::path::Path>>(path: P, config: Config) -> Result<File, Error> {
        let create = !std::fs::exists(&path).map_err(Error::IO)?;
        
        let file = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path)
            .map_err(Error::IO)?;

        let mut file = Self {
            file,
            config
        };

        if create {
            file.create_header()?;
        } else {
            file.check_if_file_valid()?;
        }

        Ok(file)
    }

    /// Read the data from a page chain. 
    pub fn read(&mut self, mut ptr: u64) -> Result<Vec<u8>, Error> {
        self.check_if_pointer_valid(ptr)?;

        let mut data = Vec::new();

        loop {
            let header = self.read_page_header(ptr)?; 
            match header {
                PageHeader::NextPage(next) => {
                    data.extend(std::iter::repeat(0).take(self.config.page_size));
                    self.file.seek(SeekFrom::Start(ptr + BYTES_IN_U64)).map_err(Error::IO)?;
                    let read_to = data.len() - self.config.page_size;
                    self.file.read(&mut data[read_to..]).map_err(Error::IO)?;
                    ptr = next;
                },
                PageHeader::FinalPage(size) => {
                    let size = size as usize;
                    data.extend(std::iter::repeat(0).take(size));
                    self.file.seek(SeekFrom::Start(ptr + BYTES_IN_U64)).map_err(Error::IO)?;
                    let read_to = data.len() - size; 
                    self.file.read(&mut data[read_to..]).map_err(Error::IO)?;
                    break;
                },
                PageHeader::DeletedPage(_) => {
                    return Err(Error::CorruptedFile);
                }
            }
        }

        Ok(data)
    }

    /// Read the root page chain.
    pub fn read_root(&mut self) -> Result<Vec<u8>, Error> {
        let root_page = self.root_page()?;
        self.read(root_page)
    }

    /// Write data to a page chain.
    pub fn write(&mut self, mut ptr: u64, mut data: &[u8]) -> Result<(), Error> {
        self.check_if_pointer_valid(ptr)?;
        
        while data.len() > self.config.page_size {
            self.file.seek(SeekFrom::Start(ptr + BYTES_IN_U64)).map_err(Error::IO)?;
            self.file.write(&data[..self.config.page_size]).map_err(Error::IO)?;
            data = &data[self.config.page_size..];
            ptr = match self.read_page_header(ptr)? {
                PageHeader::NextPage(next) => next,
                PageHeader::FinalPage(_) => {
                    let new_page = self.alloc()?;
                    self.write_page_header(ptr, PageHeader::NextPage(new_page))?;
                    new_page
                },
                PageHeader::DeletedPage(_) => {
                    return Err(Error::CorruptedFile);
                }
            }
        }

        let final_page_header = self.read_page_header(ptr)?;
        if let PageHeader::NextPage(truncated_pages) = final_page_header {
            // If there are more pages in this chain we no longer need, delete them
            self.delete(truncated_pages)?;
        }

        self.file.seek(SeekFrom::Start(ptr + BYTES_IN_U64)).map_err(Error::IO)?;
        self.file.write(data).map_err(Error::IO)?;
        self.file.write(&vec![0xFF; self.config.page_size - data.len()]).map_err(Error::IO)?; // Clear remainder of the page 
        self.write_page_header(ptr, PageHeader::FinalPage(data.len() as u64))?;

        Ok(())
    }

    /// Write to the root page chain
    pub fn write_root(&mut self, data: &[u8]) -> Result<(), Error> {
        let root_page = self.root_page()?;
        self.write(root_page, data)
    }

    /// Allocate a new page.
    /// Either takes the first page in the free list or creates a new page at the end of the file.
    /// Initializes page with a header of PageHeader::FinalPage(0). 
    pub fn alloc(&mut self) -> Result<u64, Error> {
        let free_page = self.first_free_page()?;

        let page = if free_page == 0 {
            // Create new page at the end of the file
            let new_page_ptr = self.file.seek(SeekFrom::End(0)).map_err(Error::IO)?;
            self.file.write(&vec![0xFF; self.total_page_size() as usize]).map_err(Error::IO)?;

            new_page_ptr
        } else {
            // Remove free page from chain
            let new_free_page = self.read_page_header(free_page)?;
            match new_free_page {
                PageHeader::DeletedPage(next) => {
                    self.write_u64(self.first_free_page_ptr(), next)?;
                },
                _ => return Err(Error::CorruptedFile)
            }

            free_page
        };

        self.write_page_header(page, PageHeader::FinalPage(0))?;

        Ok(page)
    }

    /// Delete a page chain.
    /// Note that this simply adds the page to the free list, without actually ever shrinking the file.
    pub fn delete(&mut self, mut ptr: u64) -> Result<(), Error> {
        self.check_if_pointer_valid(ptr)?;

        loop {
            let header = self.read_page_header(ptr)?;
            let free_pages = self.first_free_page()?;
            self.write_page_header(ptr, PageHeader::DeletedPage(free_pages))?;
            self.write_u64(self.first_free_page_ptr(), ptr)?;

            // Write garbage to the deleted page
            self.file.seek(SeekFrom::Start(ptr + BYTES_IN_U64)).map_err(Error::IO)?;
            self.file.write(&vec![0xFF; self.config.page_size]).map_err(Error::IO)?;

            match header {
                PageHeader::NextPage(next) => ptr = next,
                PageHeader::FinalPage(_) => break,
                PageHeader::DeletedPage(_) => {
                    return Err(Error::CorruptedFile);
                }
            } 
        }

        Ok(())
    }

    fn read_u64(&mut self, ptr: u64) -> Result<u64, Error> {
        self.file.seek(SeekFrom::Start(ptr as u64)).map_err(Error::IO)?;
        let mut bytes = [0; BYTES_IN_U64 as usize];
        self.file.read(&mut bytes).map_err(Error::IO)?;
        Ok(u64::from_le_bytes(bytes))
    }

    fn read_page_header(&mut self, ptr: u64) -> Result<PageHeader, Error> {
        self.read_u64(ptr).map(PageHeader::from_u64)
    }

    fn write_u64(&mut self, ptr: u64, val: u64) -> Result<(), Error> {
        self.file.seek(SeekFrom::Start(ptr)).map_err(Error::IO)?;
        self.file.write(&val.to_le_bytes()).map_err(Error::IO)?;
        Ok(())
    }

    fn write_page_header(&mut self, ptr: u64, header: PageHeader) -> Result<(), Error> {
        self.write_u64(ptr, header.to_u64())
    }

    fn magic_bytes_ptr(&self) -> u64 {
        0
    }

    fn first_free_page_ptr(&self) -> u64 {
        self.magic_bytes_ptr() + self.config.magic_bytes.len() as u64
    }

    fn header_size(&self) -> u64 {
        self.config.magic_bytes.len() as u64 + 2 * BYTES_IN_U64
    }

    fn total_page_size(&self) -> u64 {
        BYTES_IN_U64 + self.config.page_size as u64
    }

    fn root_page_ptr(&self) -> u64 {
        self.first_free_page_ptr() + BYTES_IN_U64
    }

    fn first_free_page(&mut self) -> Result<u64, Error> {
        self.read_u64(self.first_free_page_ptr())
    }

    fn root_page(&mut self) -> Result<u64, Error> {
        self.read_u64(self.root_page_ptr())
    }

    fn file_size(&self) -> Result<u64, Error> {
        self.file.metadata().map(|metadata| metadata.len()).map_err(Error::IO)
    }

    fn create_header(&mut self) -> Result<(), Error> {
        // Magic Bytes
        self.file.seek(SeekFrom::Start(self.magic_bytes_ptr())).map_err(Error::IO)?;
        self.file.write(&self.config.magic_bytes).map_err(Error::IO)?;

        // First Free Page
        self.write_u64(self.first_free_page_ptr(), 0)?;

        // Root Page
        self.write_u64(self.root_page_ptr(), 0)?;

        // Initialize Root Page Chain
        let first_root_page = self.alloc()?;
        self.write_u64(self.root_page_ptr(), first_root_page)?;

        Ok(())
    }

    fn check_if_file_valid(&mut self) -> Result<(), Error> {
        self.file.seek(SeekFrom::Start(0)).map_err(Error::IO)?;
        let mut magic_bytes = vec![0; self.config.magic_bytes.len()];
        let bytes_read = self.file.read(&mut magic_bytes).map_err(Error::IO)?;
        if bytes_read < self.config.magic_bytes.len() || self.config.magic_bytes != magic_bytes {
            return Err(Error::InvalidFile)
        }
        Ok(())
    }

    fn check_if_pointer_valid(&mut self, ptr: u64) -> Result<(), Error> {
        if ptr < self.header_size() || (ptr - self.header_size()) % self.total_page_size() != 0 {
            return Err(Error::InvalidPointer);
        }
        if ptr >= self.file_size()? {
            return Err(Error::InvalidPointer);
        }

        if matches!(self.read_page_header(ptr)?, PageHeader::DeletedPage(_)) {
            return Err(Error::DeletedPointer);
        }

        Ok(())
    }

}
