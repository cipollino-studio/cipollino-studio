
use super::{ProjectFile, FREE_PAGE_CHAIN_ADDR};

pub const FILE_PAGE_DATA_SIZE: u64 = 128;
pub const FILE_PAGE_DATA_SIZE_USIZE: usize = FILE_PAGE_DATA_SIZE as usize;
pub const FILE_PAGE_METADATA_SIZE: u64 = 8;

pub const FILE_PAGE_NEXT_PTR_OFFSET: u64 = 0;
pub const FILE_PAGE_DATA_OFFSET: u64 = FILE_PAGE_METADATA_SIZE;

impl ProjectFile {

    pub fn alloc_page(&mut self) -> Result<u64, String> {
        let first_free_page = self.read_u64(FREE_PAGE_CHAIN_ADDR)?;
        if first_free_page != 0 {
            let next_free_page = self.read_u64(first_free_page + FILE_PAGE_NEXT_PTR_OFFSET)?;
            self.write_u64(FREE_PAGE_CHAIN_ADDR, next_free_page)?;

            self.write_u64(first_free_page + FILE_PAGE_NEXT_PTR_OFFSET, 0)?;
            
            return Ok(first_free_page);
        }
        let new_page = self.file_size()?;
        self.write_u64(new_page, 0)?; // Next page pointer
        self.write(new_page + FILE_PAGE_METADATA_SIZE, &vec![0; FILE_PAGE_DATA_SIZE_USIZE])?; // Blank data

        Ok(new_page)
    }

    pub fn free_page(&mut self, page: u64) -> Result<(), String> {
        let first_free_page = self.read_u64(FREE_PAGE_CHAIN_ADDR)?;
        self.write_u64(page + FILE_PAGE_NEXT_PTR_OFFSET, first_free_page)?;
        self.write_u64(FREE_PAGE_CHAIN_ADDR, page)?;
        self.write(page + FILE_PAGE_DATA_OFFSET, &vec![0xFF; FILE_PAGE_DATA_SIZE as usize])?; // Fill page with nonsense for debugging & privacy purposes 
        Ok(())
    }

    pub fn free_page_chain(&mut self, page: u64) -> Result<(), String> {
        let mut curr_page = page;
        while curr_page != 0 {
            let next_page = self.read_u64(curr_page + FILE_PAGE_NEXT_PTR_OFFSET)?;
            self.free_page(curr_page)?;
            curr_page = next_page;
        }
        Ok(())
    }

}