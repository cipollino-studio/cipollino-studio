
use folder::Folder;
use obj::{ChildList, Obj, ObjList, ObjPtr, ObjRef};

use crate::crdt::{fractional_index::FractionalIndex, register::Register};

pub mod obj;
pub mod folder;
pub mod action;

pub struct Project {
    pub fps: f32,
    pub sample_rate: f32,

    pub folders: ObjList<Folder>,

    pub(crate) root_folder: ObjPtr<Folder>
}

impl Project {

    pub fn empty(fps: f32, sample_rate: f32) -> Self {
        Project {
            fps,
            sample_rate,
            folders: ObjList::new(),
            root_folder: ObjPtr::null(),
        }
    }

    pub fn new(fps: f32, sample_rate: f32) -> Self {
        let mut project = Self::empty(fps, sample_rate); 
        project.root_folder = ObjPtr::from_key(1);
        project.folders.objs.insert(project.root_folder, Folder {
            parent: ObjPtr::null(),
            folders: ChildList::new(),
            name: Register::new("root".to_owned(), 0)
        });
        project
    }

    pub fn root_folder(&self) -> ObjRef<Folder> {
        ObjRef {
            ptr: self.root_folder,
            obj: self.folders.get(self.root_folder).unwrap()
        }
    }

    pub(crate) fn add_with_frac_idx<T: Obj, P: Obj>(&mut self, new_obj_ptr: ObjPtr<T>, parent_ptr: ObjPtr<P>, idx: FractionalIndex, mut obj: T) -> Option<()> where T::Parent: From<ObjPtr<P>> {
        // Create the object
        *obj.parent_mut() = parent_ptr.into();
        T::obj_list_mut(self).objs.insert(new_obj_ptr, obj);

        // Add it to the parent
        let list_in_parent = T::list_in_parent_mut(self, parent_ptr.into())?;
        list_in_parent.insert(idx.clone(), new_obj_ptr);

        Some(())
    }

    pub(crate) fn add<T: Obj, P: Obj>(&mut self, new_obj_ptr: ObjPtr<T>, parent_ptr: ObjPtr<P>, idx: usize, obj: T) -> Option<FractionalIndex> where T::Parent: From<ObjPtr<P>> {
        let list_in_parent = T::list_in_parent_mut(self, parent_ptr.into())?;
        let idx = list_in_parent.get_insertion_idx(idx); 

        self.add_with_frac_idx(new_obj_ptr, parent_ptr, idx.clone(), obj)?;
        
        Some(idx) 
    }

}
