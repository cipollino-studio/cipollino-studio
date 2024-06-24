
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
            parent: Register::new((ObjPtr::null(), FractionalIndex::half()), 0),
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

    pub(crate) fn add<T: Obj<Parent = ObjPtr<P>>, P: Obj>(&mut self, new_obj_ptr: ObjPtr<T>, obj: T) -> Option<()> {
        let parent_ptr: ObjPtr<P> = obj.parent().0;
        let idx = obj.parent().1.clone();

        // Create the object
        T::obj_list_mut(self).objs.insert(new_obj_ptr, obj);

        // Add it to the parent
        let list_in_parent = T::list_in_parent_mut(self, parent_ptr.into())?;
        list_in_parent.insert(idx.clone(), new_obj_ptr);

        Some(())
    }

    pub(crate) fn get_insertion_idx<T: Obj<Parent = ObjPtr<P>>, P: Obj>(&self, parent_ptr: ObjPtr<P>, idx: usize) -> Option<FractionalIndex> {
        let list_in_parent = T::list_in_parent(self, parent_ptr.into())?;
        Some(list_in_parent.get_insertion_idx(idx)) 
    }

}
