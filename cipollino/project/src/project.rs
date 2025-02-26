
use crate::{Clip, CreateClip, CreateFolder, CreateFrame, CreateLayer, DeleteClip, DeleteFolder, DeleteFrame, DeleteLayer, Folder, Frame, Layer, RenameClip, RenameFolder, SetLayerName, TransferClip, TransferFolder, TransferLayer};

#[derive(alisa::Serializable)]
#[project(Project)]
pub struct Project {
    pub folders: alisa::UnorderedChildList<Folder>,
    pub clips: alisa::UnorderedChildList<Clip>,
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
    pub layers: alisa::ObjList<Layer>,
    pub frames: alisa::ObjList<Frame>
}

impl alisa::Project for Project {

    type Context = ();
    type Objects = Objects;

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
        alisa::ObjectKind::from::<Layer>(),
        alisa::ObjectKind::from::<Frame>(),
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
        
        alisa::OperationKind::from::<CreateLayer>(),
        alisa::OperationKind::from::<DeleteLayer>(),
        alisa::OperationKind::from::<TransferLayer>(),
        alisa::OperationKind::from::<SetLayerName>(),

        alisa::OperationKind::from::<CreateFrame>(),
        alisa::OperationKind::from::<DeleteFrame>(),
    ];

}