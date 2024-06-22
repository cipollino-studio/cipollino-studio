
use std::{fs::File, path::Path};

const MAGIC_BYTES: [u8; 8] = *b"CIPOLINO";

const MAGIC_ADDR: u64 = 0;
const VERSION_ADDR: u64 = 8;
const ROOT_OBJ_PTR_ADDR: u64 = 16;
const FREE_PAGE_CHAIN_ADDR: u64 = 32;

mod binary;
mod pages;
mod object;

pub struct ProjectFile {
    file: File,
}

impl ProjectFile {

    pub fn create(path: &Path, root_obj: bson::Bson) -> Result<Self, String> {
        if path.exists() {
            return Err("File already exists.".to_owned());
        } 
        let file = match File::options().read(true).write(true).create(true).open(path) {
            Ok(file) => file,
            Err(_) => return Err("Could not create file.".to_owned())
        };
        let mut res = Self {
            file
        };
        res.write(MAGIC_ADDR, &MAGIC_BYTES)?;
        res.write_u64(VERSION_ADDR, 1)?;
        res.write_u64(ROOT_OBJ_PTR_ADDR, 0)?;
        res.write_u64(FREE_PAGE_CHAIN_ADDR, 0)?;

        let root_ptr = res.create_obj(root_obj)?;
        res.write_u64(ROOT_OBJ_PTR_ADDR, root_ptr)?;

        Ok(res)
    }

    pub fn open(path: &Path) -> Result<Self, String> {
        let file = match File::options().read(true).write(true).create(false).open(path) {
            Ok(file) => file,
            Err(_) => return Err("Could not open file.".to_owned())
        }; 
        let mut res = Self {
            file
        };

        if res.read(MAGIC_ADDR)? != MAGIC_BYTES {
            return Err("Corrupt file.".to_owned());
        }

        Ok(res)
    }
    
}