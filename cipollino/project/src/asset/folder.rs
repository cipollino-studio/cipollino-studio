
use alisa::TreeObj;

use crate::{asset_creation_operations, asset_rename_operation, rectify_name_duplication, Action, Asset, Client, Clip, Objects, Project};

use super::deep_load_clip;

#[derive(alisa::Serializable, Clone)]
pub struct Folder {
    pub parent: alisa::Ptr<Folder>,
    pub name: String,
    pub folders: alisa::UnorderedChildList<alisa::LoadingPtr<Folder>>,
    pub clips: alisa::UnorderedChildList<alisa::LoadingPtr<Clip>>
}

impl Default for Folder {

    fn default() -> Self {
        Self {
            parent: alisa::Ptr::null(),
            name: "Folder".to_owned(),
            folders: Default::default(),
            clips: Default::default()
        }
    }

}

impl alisa::Object for Folder {

    type Project = Project;

    const NAME: &'static str = "Folder";
    const TYPE_ID: u16 = 5;

    fn list(objects: &Objects) -> &alisa::ObjList<Folder> {
        &objects.folders
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Folder> {
        &mut objects.folders
    }
}

#[derive(alisa::Serializable)]
pub struct FolderTreeData {
    pub name: String,
    pub folders: alisa::UnorderedChildListTreeData<alisa::LoadingPtr<Folder>>,
    pub clips: alisa::UnorderedChildListTreeData<alisa::LoadingPtr<Clip>>
}

impl Default for FolderTreeData {

    fn default() -> Self {
        Self {
            name: "Folder".to_owned(),
            folders: Default::default(),
            clips: Default::default()
        }
    }

}

impl alisa::TreeObj for Folder {
    type ParentPtr = alisa::Ptr<Folder>;
    type ChildList = alisa::UnorderedChildList<alisa::LoadingPtr<Folder>>;
    type TreeData = FolderTreeData;

    fn child_list<'a>(parent: alisa::Ptr<Folder>, context: &'a alisa::ProjectContext<Project>) -> Option<&'a Self::ChildList> {
        if parent.is_null() {
            return Some(&context.project().folders);
        }
        context.obj_list().get(parent).map(|folder| &folder.folders)
    }

    fn child_list_mut<'a>(parent: alisa::Ptr<Folder>, context: &'a mut alisa::Recorder<Project>) -> Option<&'a mut Self::ChildList> {
        if parent.is_null() {
            return Some(&mut context.project_mut().folders);
        }
        context.get_obj_mut(parent).map(|folder| &mut folder.folders)
    }

    fn parent(&self) -> alisa::Ptr<Folder> {
        self.parent
    }

    fn parent_mut(&mut self) -> &mut alisa::Ptr<Folder> {
        &mut self.parent
    }

    fn instance(data: &Self::TreeData, ptr: alisa::Ptr<Self>, parent: alisa::Ptr<Folder>, recorder: &mut alisa::Recorder<Project>) {
        let folder = Self {
            parent,
            name: data.name.clone(),
            folders: data.folders.instance(ptr, recorder),
            clips: data.clips.instance(ptr, recorder),
        };
        recorder.add_obj(ptr, folder);
    }

    fn destroy(&self, recorder: &mut alisa::Recorder<Project>) {
        self.folders.destroy(recorder);
        self.clips.destroy(recorder);
    }

    fn collect_data(&self, objects: &Objects) -> Self::TreeData {
        FolderTreeData {
            name: self.name.clone(),
            folders: self.folders.collect_data(objects),
            clips: self.clips.collect_data(objects)
        }
    }

    fn can_delete(ptr: alisa::Ptr<Self>, project: &alisa::ProjectContext<Project>, source: alisa::OperationSource) -> bool {
        let Some(folder) = project.obj_list().get(ptr) else {
            return false;
        };
        for folder_ptr in folder.folders.iter() {
            if !Folder::can_delete(folder_ptr.ptr(), project, source) {
                return false;
            }
        }
        for clip_ptr in folder.clips.iter() {
            if !Clip::can_delete(clip_ptr.ptr(), project, source) {
                return false;
            }
        }
        true
    }

}

impl Asset for Folder {

    fn name(&self) -> &String {
        &self.name
    }

    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    
    fn rename(action: &mut Action, ptr: alisa::Ptr<Self>, name: String) {
        action.push(RenameFolder {
            ptr,
            name,
        });
    }

    fn delete(action: &mut Action, ptr: alisa::Ptr<Self>) {
        action.push(DeleteFolder {
            ptr
        }); 
    }

}

asset_creation_operations!(Folder);
asset_rename_operation!(Folder);

/*
    We need a custom transfer operation to account for the possibility that a folder
    could be transferred into a child folder, which should be impossible.
*/
#[derive(alisa::Serializable)]
pub struct TransferFolder {
    pub ptr: alisa::Ptr<Folder>,
    pub new_parent: alisa::Ptr<Folder>
}

impl Default for TransferFolder {

    fn default() -> Self {
        Self {
            ptr: alisa::Ptr::null(),
            new_parent: alisa::Ptr::null()
        }
    }

}

fn is_inside_folder(recorder: &alisa::Recorder<Project>, parent_ptr: alisa::Ptr<Folder>, child_ptr: alisa::Ptr<Folder>) -> bool {
    if parent_ptr == child_ptr {
        return true;
    }
    if let Some(child) = recorder.get_obj(child_ptr) {
        return is_inside_folder(recorder, parent_ptr, child.parent);
    }
    false
}

impl alisa::Operation for TransferFolder {

    type Project = Project;
    const NAME: &'static str = "TransferFolder";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Project>) -> bool {

        // Make sure we're not moving the folder somewhere inside itself
        if is_inside_folder(recorder, self.ptr, self.new_parent) {
            return false;
        }

        if alisa::transfer_tree_object(recorder, self.ptr, &self.new_parent, &()) {
            // Fix the name of the folder
            let context = recorder.context();
            let Some(child_list) = Folder::child_list(self.new_parent, &context) else { return false; };
            let sibling_names = Folder::get_sibling_names(child_list, recorder, Some(self.ptr));
            rectify_name_duplication(self.ptr, sibling_names, recorder);

            true
        } else {
            false
        }

    }

    
}

impl alisa::InvertibleOperation for TransferFolder {

    type Inverse = Self;

    fn inverse(&self, context: &alisa::ProjectContext<Project>) -> Option<Self> {
        let folder = context.obj_list().get(self.ptr)?;
        Some(Self {
            ptr: self.ptr,
            new_parent: folder.parent,
        })
    }

}

pub fn deep_load_folder(folder_ptr: alisa::Ptr<Folder>, client: &Client) {
    let Some(folder) = client.get(folder_ptr) else {
        return;
    };
    
    for subfolder in folder.folders.iter() {
        deep_load_folder(subfolder.ptr(), client);
    }
    for clip in folder.clips.iter() {
        deep_load_clip(clip.ptr(), client);
    }
}