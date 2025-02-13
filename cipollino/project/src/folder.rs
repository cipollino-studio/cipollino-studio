
use crate::{asset_operations, Asset, Objects, Project};

#[derive(alisa::Serializable, Clone)]
#[project(Project)]
pub struct Folder {
    parent: alisa::Ptr<Folder>,
    name: String,
    folders: alisa::UnorderedChildList<Folder>
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
    name: String,
    folders: alisa::UnorderedChildListTreeData<Folder>
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

}

asset_operations!(Folder);
