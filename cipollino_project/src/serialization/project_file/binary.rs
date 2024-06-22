
use std::io::{Read, Seek, SeekFrom, Write};

use super::ProjectFile;

impl ProjectFile {

    fn cursor_to(&mut self, ptr: u64) -> Result<(), String> {
        self.file.seek(SeekFrom::Start(ptr)).map(|_| ()).map_err(|err| err.to_string())
    }
    
    pub fn write(&mut self, ptr: u64, data: &[u8]) -> Result<(), String> {
        self.cursor_to(ptr)?;
        self.file.write(data).map_err(|err| err.to_string())?;
        Ok(())
    }
    
    pub fn write_u64(&mut self, ptr: u64, val: u64) -> Result<(), String> {
        self.write(ptr, &val.to_le_bytes())?; 
        Ok(())
    }

    pub fn file_size(&mut self) -> Result<u64, String> {
        self.file.seek(SeekFrom::End(0)).map_err(|err| err.to_string())
    }

    pub fn read<const N: usize>(&mut self, ptr: u64) -> Result<[u8; N], String> {
        self.cursor_to(ptr)?;
        let mut res = [0; N];
        self.file.read(&mut res).map_err(|err| err.to_string())?;
        Ok(res)
    }

    pub fn read_u64(&mut self, ptr: u64) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.read(ptr)?))
    }

}