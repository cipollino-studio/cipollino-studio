
use std::cell::RefCell;
use crate::AssetList;

pub struct ProjectState {
    pub client: project::Client,

    assets_to_delete: RefCell<Vec<AssetList>>
}

impl ProjectState {

    pub fn new(client: project::Client) -> Self {
        Self {
            client,
            assets_to_delete: RefCell::new(Vec::new())
        }
    }

    pub fn delete_assets(&self, selection: AssetList) {
        self.assets_to_delete.borrow_mut().push(selection);
    }

    pub fn tick(&self) {
        let to_delete = self.assets_to_delete.borrow_mut().pop();
        if let Some(to_delete) = to_delete {
            if !to_delete.try_delete(&self.client) {
                to_delete.deep_load_all(&self.client);
                self.assets_to_delete.borrow_mut().push(to_delete);
            }
        }
    }

}

