
use super::{pages::{FILE_PAGE_DATA_OFFSET, FILE_PAGE_DATA_SIZE_USIZE, FILE_PAGE_NEXT_PTR_OFFSET}, ProjectFile, ROOT_OBJ_PTR_ADDR};

impl ProjectFile {

    pub fn get_obj_data(&mut self, first_page_ptr: u64) -> Result<bson::Bson, String> {
        let mut bytes = Vec::new();
        let mut curr_page = first_page_ptr;
        loop {
            let page_data = self.read::<FILE_PAGE_DATA_SIZE_USIZE>(curr_page + FILE_PAGE_DATA_OFFSET)?;
            bytes.extend_from_slice(&page_data);
            curr_page = self.read_u64(curr_page + FILE_PAGE_NEXT_PTR_OFFSET)?;
            if curr_page == 0 {
                break;
            }
        }
        Ok(bson::from_slice(&bytes).map_err(|_| "Could not decode bson data.".to_owned())?)
    } 

    pub fn set_obj_data(&mut self, first_page_ptr: u64, data: bson::Bson) -> Result<(), String> {
        let data_bytes = bson::to_vec(&data).unwrap(); 
        let mut data_bytes = data_bytes.as_slice();
        let mut curr_page = first_page_ptr;
        loop {
            let (curr_page_data, rest) = data_bytes.split_at(FILE_PAGE_DATA_SIZE_USIZE.min(data_bytes.len()));
            self.write(curr_page + FILE_PAGE_DATA_OFFSET, curr_page_data)?;

            data_bytes = rest; 
            if data_bytes.len() == 0 {
                break;
            }
            let next_page = self.read_u64(curr_page + FILE_PAGE_NEXT_PTR_OFFSET)?;
            if next_page == 0 {
                let new_page = self.alloc_page()?;
                self.write_u64(curr_page + FILE_PAGE_NEXT_PTR_OFFSET, new_page)?;
                curr_page = new_page;
            } else {
                curr_page = next_page;
            }
        }

        let remaining_pages = self.read_u64(curr_page + FILE_PAGE_NEXT_PTR_OFFSET)?;
        if remaining_pages != 0 {
            self.free_page_chain(remaining_pages)?;
            self.write_u64(curr_page + FILE_PAGE_NEXT_PTR_OFFSET, 0)?;
        }

        Ok(())
    }

    pub fn create_obj(&mut self, data: bson::Bson) -> Result<u64, String> {
        let ptr = self.alloc_page()?;
        self.set_obj_data(ptr, data)?;
        Ok(ptr)
    }

    pub fn delete_obj(&mut self, first_page_ptr: u64) -> Result<(), String> {
        self.free_page_chain(first_page_ptr)
    }

    pub fn set_root_obj(&mut self, data: bson::Bson) -> Result<(), String> {
        let root_obj_ptr = self.read_u64(ROOT_OBJ_PTR_ADDR)?;
        self.set_obj_data(root_obj_ptr, data)?;
        Ok(())
    }

    pub fn get_root_obj(&mut self) -> Result<bson::Bson, String> {
        let root_obj_ptr = self.read_u64(ROOT_OBJ_PTR_ADDR)?;
        self.get_obj_data(root_obj_ptr)
    }

}