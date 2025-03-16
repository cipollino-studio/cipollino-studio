
use crate::{Clip, ClipInner, CreateClip, CreateClipInner, CreateFolder, CreateFrame, CreateLayer, CreateStroke, DeleteClip, DeleteFolder, DeleteFrame, DeleteLayer, DeleteStroke, Folder, Frame, Layer, RenameClip, RenameFolder, SetClipInnerLength, SetFrameTime, SetLayerName, Stroke, TransferClip, TransferFolder, TransferLayer};

#[derive(alisa::Serializable)]
#[project(Project)]
pub struct Project {
    pub folders: alisa::UnorderedChildList<alisa::LoadingPtr<Folder>>,
    pub clips: alisa::UnorderedChildList<alisa::LoadingPtr<Clip>>,
}

impl Default for Project {

    fn default() -> Self {
        Self {
            folders: Default::default(),
            clips: Default::default()
        }
    }

}

#[derive(Default)]
pub struct Objects { 
    pub folders: alisa::ObjList<Folder>,
    pub clips: alisa::ObjList<Clip>,
    pub clip_inners: alisa::ObjList<ClipInner>,
    pub layers: alisa::ObjList<Layer>,
    pub frames: alisa::ObjList<Frame>,
    pub strokes: alisa::ObjList<Stroke>
}

pub struct ActionContext {
    pub name: String
}

impl ActionContext {

    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into()
        }
    }

}

impl alisa::Project for Project {

    type Context = ();
    type Objects = Objects;
    type ActionContext = ActionContext;

    fn empty() -> Self {
        Self::default()
    }

    fn create_default(&mut self) {

    }

    fn verter_config() -> alisa::verter::Config {
        alisa::verter::Config {
            magic_bytes: b"CIPOLINO",
            page_size: 64,
        } 
    }

    const OBJECTS: &'static [alisa::ObjectKind<Self>] = &[
        alisa::ObjectKind::from::<Folder>(),
        alisa::ObjectKind::from::<Clip>(),
        alisa::ObjectKind::from::<ClipInner>(),
        alisa::ObjectKind::from::<Layer>(),
        alisa::ObjectKind::from::<Frame>(),
        alisa::ObjectKind::from::<Stroke>(),
    ];

    const OPERATIONS: &'static [alisa::OperationKind<Self>] = &[
        alisa::OperationKind::from::<CreateFolder>(),
        alisa::OperationKind::from::<DeleteFolder>(),
        alisa::OperationKind::from::<RenameFolder>(),
        alisa::OperationKind::from::<TransferFolder>(),

        alisa::OperationKind::from::<CreateClip>(),
        alisa::OperationKind::from::<DeleteClip>(),
        alisa::OperationKind::from::<RenameClip>(),
        alisa::OperationKind::from::<TransferClip>(),

        alisa::OperationKind::from::<CreateClipInner>(),
        alisa::OperationKind::from::<SetClipInnerLength>(),
        
        alisa::OperationKind::from::<CreateLayer>(),
        alisa::OperationKind::from::<DeleteLayer>(),
        alisa::OperationKind::from::<TransferLayer>(),
        alisa::OperationKind::from::<SetLayerName>(),

        alisa::OperationKind::from::<CreateFrame>(),
        alisa::OperationKind::from::<DeleteFrame>(),
        alisa::OperationKind::from::<SetFrameTime>(),

        alisa::OperationKind::from::<CreateStroke>(),
        alisa::OperationKind::from::<DeleteStroke>(),
    ];

}
