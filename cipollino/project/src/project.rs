
use crate::{Clip, ClipInner, ClipTreeData, CreateClip, CreateClipInner, CreateFolder, CreateFrame, CreateLayer, CreateStroke, DeleteClip, DeleteFolder, DeleteFrame, DeleteLayer, DeleteStroke, Folder, Frame, Layer, LayerParent, LayerTreeData, RenameClip, RenameFolder, SetClipInnerFramerate, SetClipInnerHeight, SetClipInnerLength, SetClipInnerWidth, SetFrameTime, SetLayerName, SetStrokeColor, SetStrokeStroke, Stroke, TransferClip, TransferFolder, TransferLayer};

#[derive(alisa::Serializable, Clone)]
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

#[derive(Clone)]
pub struct ActionContext {
    pub name: String,
    pub open_clip: alisa::Ptr<Clip>,
    pub time: f32
}

impl ActionContext {

    pub fn new<S: Into<String>>(name: S, open_clip: alisa::Ptr<Clip>, time: f32) -> Self {
        Self {
            name: name.into(),
            open_clip,
            time
        }
    }

}

impl alisa::Project for Project {

    type Objects = Objects;
    type ActionContext = ActionContext;

    fn empty() -> Self {
        Self::default()
    }

    fn create_default(client: &alisa::Client<Self>) {
        let default_clip_ptr = client.next_ptr();
        let default_clip_inner_ptr = client.next_ptr();
        let layer_ptr = client.next_ptr();
        client.queue_operation(CreateClip {
            ptr: default_clip_ptr,
            parent: alisa::Ptr::null(),
            data: ClipTreeData {
                inner_ptr: default_clip_inner_ptr,
                ..Default::default()
            },
        });
        client.queue_operation(CreateLayer {
            ptr: layer_ptr,
            parent: LayerParent::Clip(default_clip_ptr),
            idx: 0,
            data: LayerTreeData::default(),
        });
    }

    fn verter_config() -> alisa::verter::Config {
        alisa::verter::Config {
            magic_bytes: b"CIPOLINO",
            page_size: 64,
        } 
    }

    const OBJECTS: &'static [alisa::ObjectKind<Self>] = &[
        alisa::ObjectKind::from::<Stroke>(),
        alisa::ObjectKind::from::<Frame>(),
        alisa::ObjectKind::from::<Layer>(),
        alisa::ObjectKind::from::<Clip>(),
        alisa::ObjectKind::from::<ClipInner>(),
        alisa::ObjectKind::from::<Folder>(),
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
        alisa::OperationKind::from::<SetClipInnerWidth>(),
        alisa::OperationKind::from::<SetClipInnerHeight>(),
        alisa::OperationKind::from::<SetClipInnerLength>(),
        alisa::OperationKind::from::<SetClipInnerFramerate>(),
        
        alisa::OperationKind::from::<CreateLayer>(),
        alisa::OperationKind::from::<DeleteLayer>(),
        alisa::OperationKind::from::<TransferLayer>(),
        alisa::OperationKind::from::<SetLayerName>(),

        alisa::OperationKind::from::<CreateFrame>(),
        alisa::OperationKind::from::<DeleteFrame>(),
        alisa::OperationKind::from::<SetFrameTime>(),

        alisa::OperationKind::from::<CreateStroke>(),
        alisa::OperationKind::from::<DeleteStroke>(),
        alisa::OperationKind::from::<SetStrokeStroke>(),
        alisa::OperationKind::from::<SetStrokeColor>(),
    ];

}
