
use crate::{crdt::register::Register, project::{action::{Action, ObjAction}, folder::Folder, obj::{ChildList, ObjPtr}, Project}, protocol::Message};

use super::ProjectClient;

impl ProjectClient {

    pub fn add_folder(&mut self, project: &mut Project, parent: ObjPtr<Folder>, name: String) -> Option<()> {
        let ptr = ObjPtr::from_key(self.next_key()?);
        let name = Register::new(name, self.client_id());
        let name_update = name.to_update();
        let parent_reg = Register::new((parent, project.get_insertion_idx::<Folder, _>(parent, 0)?), self.client_id());
        let parent_update = parent_reg.to_update();
        project.add(ptr, Folder {
            parent: parent_reg,
            folders: ChildList::new(),
            name 
        })?;

        match self {
            ProjectClient::Local(local) => {
                local.serializer.set_obj_data(project, ptr);
                local.serializer.set_obj_data(project, parent);
                local.update_root_obj(project);
            },
            ProjectClient::Collab(collab) => {
                collab.socket.send(Message::AddFolder {
                    ptr,
                    name: name_update,
                    parent: parent_update 
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
                local.serializer.set_obj_data(project, folder_ptr);
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
        action.add_act(ObjAction::SetFolderName(folder_ptr, old_name));
        Some(())
    }

    pub fn transfer_folder_no_action(&mut self, project: &mut Project, folder_ptr: ObjPtr<Folder>, new_parent: ObjPtr<Folder>) -> Option<()> {
        if Folder::is_inside(project, folder_ptr, new_parent) {
            return None;
        }

        let new_folder = project.folders.get_mut(new_parent)?;
        let idx = new_folder.folders.get_insertion_idx(0);

        let folder = project.folders.get_mut(folder_ptr)?;
        let old_parent = folder.parent.0; 
        let update = folder.parent.set((new_parent, idx.clone()))?;

        project.folders.get_mut(old_parent)?.folders.remove(folder_ptr);

        let new_folder = project.folders.get_mut(new_parent)?;
        new_folder.folders.insert(idx, folder_ptr);

        match self {
            ProjectClient::Local(local) => {
                local.serializer.set_obj_data(project, folder_ptr);
                local.serializer.set_obj_data(project, old_parent);
                local.serializer.set_obj_data(project, new_parent);
            },
            ProjectClient::Collab(collab) => {
                collab.socket.send(Message::TransferFolder {
                    ptr: folder_ptr,
                    parent_update: update
                });
            },
        };

        Some(())
    }

    pub fn transfer_folder(&mut self, project: &mut Project, folder_ptr: ObjPtr<Folder>, new_parent: ObjPtr<Folder>, action: &mut Action) -> Option<()> {
        let old_parent = project.folders.get(folder_ptr)?.parent.value.0; 
        self.transfer_folder_no_action(project, folder_ptr, new_parent);
        action.add_act(ObjAction::TransferFolder(folder_ptr, old_parent));
        Some(())
    }

}
