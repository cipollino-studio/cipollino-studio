
use alisa::TreeObj;

use crate::{asset_creation_operations, asset_rename_operation, rectify_name_duplication, Action, Asset, Client, Clip, Objects, Project};

#[derive(alisa::Serializable, Clone)]
#[project(Project)]
pub struct Folder {
    pub parent: alisa::Ptr<Folder>,
    pub name: String,
    pub folders: alisa::UnorderedChildList<Folder>,
    pub clips: alisa::UnorderedChildList<Clip>
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

    fn list(objects: &Objects) -> &alisa::ObjList<Folder> {
        &objects.folders
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Folder> {
        &mut objects.folders
    }
}

#[derive(alisa::Serializable)]
#[project(Project)]
pub struct FolderTreeData {
    pub name: String,
    pub folders: alisa::UnorderedChildListTreeData<Folder>,
    pub clips: alisa::UnorderedChildListTreeData<Clip>
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
    type ChildList = alisa::UnorderedChildList<Folder>;
    type TreeData = FolderTreeData;

    fn child_list<'a>(parent: alisa::Ptr<Folder>, context: &'a alisa::ProjectContext<Project>) -> Option<&'a Self::ChildList> {
        if parent.is_null() {
            return Some(&context.project().folders);
        }
        context.obj_list().get(parent).map(|folder| &folder.folders)
    }

    fn child_list_mut<'a>(parent: alisa::Ptr<Folder>, context: &'a mut alisa::ProjectContextMut<Project>) -> Option<&'a mut Self::ChildList> {
        if parent.is_null() {
            return Some(&mut context.project_mut().folders);
        }
        context.obj_list_mut().get_mut(parent).map(|folder| &mut folder.folders)
    }

    fn parent(&self) -> alisa::Ptr<Folder> {
        self.parent
    }

    fn parent_mut(&mut self) -> &mut alisa::Ptr<Folder> {
        &mut self.parent
    }

    fn instance(data: &Self::TreeData, ptr: alisa::Ptr<Self>, parent: alisa::Ptr<Folder>, recorder: &mut alisa::Recorder<Project>) {
        use alisa::Object;
        let folder = Self {
            parent,
            name: data.name.clone(),
            folders: data.folders.instance(ptr, recorder),
            clips: data.clips.instance(ptr, recorder),
        };
        Self::add(recorder, ptr, folder)
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

}

impl Asset for Folder {

    fn name(&self) -> &String {
        &self.name
    }

    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    
    fn rename(client: &Client, action: &mut Action, ptr: alisa::Ptr<Self>, name: String) {
        client.perform(action, RenameFolder {
            ptr,
            name,
        });
    }

    fn delete(client: &Client, action: &mut Action, ptr: alisa::Ptr<Self>) {
        client.perform(action, DeleteFolder {
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
#[project(Project)]
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

fn is_inside_folder(folders: &alisa::ObjList<Folder>, parent_ptr: alisa::Ptr<Folder>, child_ptr: alisa::Ptr<Folder>) -> bool {
    if parent_ptr == child_ptr {
        return true;
    }
    if let Some(child) = folders.get(child_ptr) {
        return is_inside_folder(folders, parent_ptr, child.parent);
    }
    false
}

impl alisa::Operation for TransferFolder {

    type Project = Project;
    type Inverse = Self;
    const NAME: &'static str = "TransferFolder";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Project>) {

        // Make sure we're not moving the folder somewhere inside itself
        if is_inside_folder(recorder.obj_list(), self.ptr, self.new_parent) {
            return;
        }

        if alisa::transfer_tree_object(recorder, self.ptr, &self.new_parent, &()).is_some() {
            // Fix the name of the folder
            let context = recorder.context();
            let Some(child_list) = Folder::child_list(self.new_parent, &context) else { return; };
            let sibling_names = Folder::get_sibling_names(child_list, recorder.obj_list(), Some(self.ptr));
            rectify_name_duplication(self.ptr, sibling_names, recorder);
        }

    }

    fn inverse(&self, context: &alisa::ProjectContext<Project>) -> Option<Self> {
        let folder = context.obj_list().get(self.ptr)?;
        Some(Self {
            ptr: self.ptr,
            new_parent: folder.parent,
        })
    }
}
