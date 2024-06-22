
use crate::{crdt::register::Register, project::{action::{Action, ObjAction}, folder::Folder, obj::{ChildList, ObjPtr}, Project}, protocol::Message};

use super::ProjectClient;

impl ProjectClient {

    pub fn add_folder(&mut self, project: &mut Project, parent: ObjPtr<Folder>, name: String) -> Option<()> {
        let ptr = ObjPtr::from_key(self.next_key()?);
        let name_reg = Register::new(name, self.client_id());
        let idx = project.add(ptr, parent, 0, Folder {
            parent,
            folders: ChildList::new(),
            name: name_reg.clone() 
        })?;

        match self {
            ProjectClient::Local(local) => {
                local.set_obj_data(project, ptr);
                local.update_root_obj(project);
            },
            ProjectClient::Collab(collab) => {
                collab.socket.send(Message::AddFolder {
                    ptr,
                    idx,
                    name: name_reg.to_update(),
                    parent
                })
            },
        }

        Some(())
    }

    pub fn set_folder_name_no_action(&mut self, project: &mut Project, folder_ptr: ObjPtr<Folder>, name: String) -> Option<()> {
        let folder = project.folders.get_mut(folder_ptr)?;
        let update = folder.name.set(name)?;

        match self {
            ProjectClient::Local(local) => {
                local.set_obj_data(project, folder_ptr);
            },
            ProjectClient::Collab(collab) => {
                collab.socket.send(Message::SetFolderName {
                    ptr: folder_ptr,
                    name_update: update
                });
            },
        }

        Some(())
    }

    pub fn set_folder_name(&mut self, project: &mut Project, folder_ptr: ObjPtr<Folder>, name: String, action: &mut Action) -> Option<()> {
        let old_name = project.folders.get(folder_ptr)?.name.value.clone(); 
        self.set_folder_name_no_action(project, folder_ptr, name.clone());
        action.add_act(ObjAction::SetFolderName(folder_ptr, old_name, name));
        Some(())
    }

}