
use super::obj::{Obj, ObjList};

include!("folder.gen.rs");

impl Obj for Folder {

    fn obj_list(project: &Project) -> &ObjList<Self> {
        &project.folders
    }

    fn obj_list_mut(project: &mut Project) -> &mut ObjList<Self> {
        &mut project.folders
    }

} 

impl Folder {

    /**
        Returns true if child is inside parent
     */
    pub fn is_inside(project: &Project, parent: ObjPtr<Folder>, child: ObjPtr<Folder>) -> bool {
        if child == parent {
            return true;
        }
        if child == ObjPtr::null() {
            return false;
        }
        
        let Some(child) = project.folders.get(child) else { return false; }; 
        Self::is_inside(project, parent, child.folder.0)
    }

}
