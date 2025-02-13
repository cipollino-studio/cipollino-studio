

use alisa::Object;

use super::{Project, ProjectObjects};

#[derive(Clone, alisa::Serializable)]
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
            folders: alisa::UnorderedChildList::default()
        }
    }

}

impl alisa::Object for Folder {

    type Project = Project;

    const NAME: &'static str = "Folder";

    fn list(objects: &ProjectObjects) -> &alisa::ObjList<Self> {
        &objects.folders
    }

    fn list_mut(objects: &mut ProjectObjects) -> &mut alisa::ObjList<Self> {
        &mut objects.folders
    }

}

impl alisa::TreeObj for Folder {
    type ParentPtr = alisa::Ptr<Folder>;
    type ChildList = alisa::UnorderedChildList<Folder>;
    type TreeData = FolderTreeData;

    fn child_list<'a>(parent: Self::ParentPtr, context: &'a alisa::ProjectContext<Project>) -> Option<&'a Self::ChildList> {
        if parent.is_null() {
            return Some(&context.project().folders);
        }
        Some(&context.obj_list().get(parent)?.folders)
    }

    fn child_list_mut<'a>(parent: Self::ParentPtr, recorder: &'a mut alisa::ProjectContextMut<Self::Project>) -> Option<&'a mut Self::ChildList> {
        if parent.is_null() {
            return Some(&mut recorder.project_mut().folders);
        }
        Some(&mut recorder.obj_list_mut().get_mut(parent)?.folders)
    }

    fn parent(&self) -> Self::ParentPtr {
        self.parent
    }

    fn parent_mut(&mut self) -> &mut Self::ParentPtr {
        &mut self.parent
    }

    fn instance(data: &Self::TreeData, ptr: alisa::Ptr<Self>, parent: Self::ParentPtr, recorder: &mut alisa::Recorder<Self::Project>) {
        let object = Self {
            parent: parent,
            name: data.name.clone(),
            folders: data.folders.instance(ptr, recorder),
        };
        Self::add(recorder, ptr, object);
    }

    fn destroy(&self, recorder: &mut alisa::Recorder<Self::Project>) {
        self.folders.destroy(recorder);
    } 

    fn collect_data(&self, objects: &<Self::Project as alisa::Project>::Objects) -> Self::TreeData {
        FolderTreeData {
            name: self.name.clone(),
            folders: self.folders.collect_data(objects),
        }
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
            name: "Folder".to_string(),
            folders: alisa::UnorderedChildListTreeData::default() 
        }
    }

}

alisa::tree_object_operations!(Folder);
alisa::object_set_property_operation!(Folder, name, String);
