
use alisa::{Children, TreeObj};

use crate::{asset_operations, rectify_name_duplication, Action, Asset, Client, Objects, Project};

#[derive(alisa::Serializable, Clone)]
#[project(Project)]
pub struct Folder {
    pub parent: alisa::Ptr<Folder>,
    pub name: String,
    pub folders: alisa::UnorderedChildList<Folder>
}

impl Default for Folder {

    fn default() -> Self {
        Self {
            parent: alisa::Ptr::null(),
            name: "Folder".to_owned(),
            folders: Default::default()
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
    pub folders: alisa::UnorderedChildListTreeData<Folder>
}

impl Default for FolderTreeData {

    fn default() -> Self {
        Self {
            name: "Folder".to_owned(),
            folders: Default::default()
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
        };
        Self::add(recorder, ptr, folder)
    }

    fn destroy(&self, recorder: &mut alisa::Recorder<Project>) {
        self.folders.destroy(recorder);
    }

    fn collect_data(&self, objects: &Objects) -> Self::TreeData {
        FolderTreeData {
            name: self.name.clone(),
            folders: self.folders.collect_data(objects),
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

asset_operations!(Folder);

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

        // Make sure we have everything we need
        let Some(obj) = recorder.obj_list().get(self.ptr) else { return; };
        let old_parent_ptr = obj.parent;
        if Folder::child_list(old_parent_ptr, &recorder.context()).is_none() {
            return;
        }
        let context = recorder.context();
        let Some(new_child_list) = Folder::child_list(self.new_parent, &context) else { return; };
        let sibling_names = Folder::get_sibling_names(new_child_list, recorder.obj_list(), Some(self.ptr));
        if is_inside_folder(recorder.obj_list(), self.ptr, self.new_parent) {
            return;
        }

        // Set the object's parent
        let Some(obj) = recorder.obj_list_mut().get_mut(self.ptr) else { return; };
        obj.parent = self.new_parent;
        recorder.push_delta(alisa::SetParentDelta {
            ptr: self.ptr,
            new_parent: old_parent_ptr,
        });

        // Remove the object from the old parent's child list
        if let Some(old_child_list) = Folder::child_list_mut(old_parent_ptr, recorder.context_mut()) {
            old_child_list.remove(self.ptr);
            recorder.push_delta(alisa::InsertChildDelta {
                parent: old_parent_ptr,
                ptr: self.ptr,
                idx: (),
            });
        }

        // Add the object to the new parent's child list
        if let Some(new_child_list) = Folder::child_list_mut(self.new_parent, recorder.context_mut()) {
            new_child_list.insert((), self.ptr);
            recorder.push_delta(alisa::RemoveChildDelta {
                parent: self.new_parent,
                ptr: self.ptr,
            });
        }

        // Fix the name of the folder
        rectify_name_duplication(self.ptr, sibling_names, recorder);
    }

    fn inverse(&self, context: &alisa::ProjectContext<Project>) -> Option<Self> {
        let folder = context.obj_list().get(self.ptr)?;
        Some(Self {
            ptr: self.ptr,
            new_parent: folder.parent,
        })
    }
}
