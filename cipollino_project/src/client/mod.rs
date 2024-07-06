
use collab::Collab;
use local::Local;

use crate::project::Project;

pub mod collab;
pub mod local;
pub enum ProjectClient {
    Local(Local),
    Collab(Collab) 
}

impl ProjectClient {

    pub fn is_collab(&self) -> bool {
        match self {
            ProjectClient::Local { .. } => false,
            ProjectClient::Collab { .. } => true,
        }
    }

    pub(crate) fn next_key(&mut self) -> Option<u64> {
        match self {
            ProjectClient::Local(local) => Some(local.next_key()),
            ProjectClient::Collab(collab) => collab.next_key(),
        }
    }

    pub fn has_keys(&self) -> bool {
        match self {
            ProjectClient::Local(_local) => true,
            ProjectClient::Collab(collab) => collab.has_keys(),
        }
    }

    pub fn update(&mut self, project: &mut Project) -> Result<(), String> {
        match self {
            ProjectClient::Local(_local) => {
                Ok(())
            },
            ProjectClient::Collab(collab) => collab.update(project),
        }
    }

    pub fn client_id(&self) -> u64 {
        match self {
            ProjectClient::Local(_local) => 0,
            ProjectClient::Collab(collab) => collab.client_id,
        }
    }

}

include!("client.gen.rs");

impl ProjectClient {

    pub fn transfer_folder_no_action(&mut self, project: &mut Project, ptr: ObjPtr<Folder>, new_parent_ptr: ObjPtr<Folder>, idx: FractionalIndex) -> Option<()> {
        if Folder::is_inside(project, ptr, new_parent_ptr) {
            return None;
        }
        self.transfer_folder_no_action_gen(project, ptr, new_parent_ptr, idx)
    }

}