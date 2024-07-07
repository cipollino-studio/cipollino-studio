
use clip::Clip;
use folder::Folder;
use frame::Frame;
use layer::Layer;
use obj::{ChildList, ObjList, ObjPtr, ObjRef, ObjState};

use crate::crdt::{fractional_index::FractionalIndex, register::Register};

pub mod obj;
pub mod folder;
pub mod clip;
pub mod layer;
pub mod action;
pub mod frame;

pub struct Project {
    pub fps: f32,
    pub sample_rate: f32,

    pub folders: ObjList<Folder>,
    pub clips: ObjList<Clip>,
    pub layers: ObjList<Layer>,
    pub frames: ObjList<Frame>,

    pub(crate) root_folder: ObjPtr<Folder>
}

impl Project {

    pub fn empty(fps: f32, sample_rate: f32) -> Self {
        Project {
            fps,
            sample_rate,
            folders: ObjList::new(),
            clips: ObjList::new(),
            layers: ObjList::new(),
            frames: ObjList::new(),
            root_folder: ObjPtr::null(),
        }
    }

    pub fn new(fps: f32, sample_rate: f32) -> Self {
        let mut project = Self::empty(fps, sample_rate); 
        project.root_folder = ObjPtr::from_key(1);
        project.folders.objs.insert(project.root_folder, ObjState::Loaded(Folder {
            folder: Register::new((ObjPtr::null(), FractionalIndex::half()), 0),
            folders: ChildList::new(),
            clips: ChildList::new(),
            name: Register::new("root".to_owned(), 0)
        }));
        project
    }

    pub fn root_folder(&self) -> ObjRef<Folder> {
        ObjRef {
            ptr: self.root_folder,
            obj: self.folders.get(self.root_folder).unwrap()
        }
    }

}
