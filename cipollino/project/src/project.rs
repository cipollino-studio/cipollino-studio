
use crate::{Clip, ClipInner, ClipTreeData, Color, CreateClip, CreateClipInner, CreateColor, CreateFill, CreateFolder, CreateFrame, CreateLayer, CreateLayerGroup, CreatePalette, CreatePaletteInner, CreateStroke, DeleteClip, DeleteColor, DeleteFill, DeleteFolder, DeleteFrame, DeleteLayer, DeleteLayerGroup, DeletePalette, DeleteStroke, Fill, Folder, Frame, Layer, LayerGroup, LayerParent, LayerTreeData, Palette, PaletteInner, RenameClip, RenameFolder, RenamePalette, SetClipInnerBackgroundColor, SetClipInnerFramerate, SetClipInnerHeight, SetClipInnerLength, SetClipInnerWidth, SetColorColor, SetColorName, SetFillColor, SetFillPaths, SetFrameTime, SetLayerGroupName, SetLayerName, SetStrokeColor, SetStrokeStroke, Stroke, TransferClip, TransferFolder, TransferLayer, TransferLayerGroup, TransferPalette};

#[derive(alisa::Serializable, Clone)]
pub struct Project {
    pub folders: alisa::UnorderedChildList<alisa::LoadingPtr<Folder>>,
    pub clips: alisa::UnorderedChildList<alisa::LoadingPtr<Clip>>,
    pub palettes: alisa::UnorderedChildList<alisa::LoadingPtr<Palette>>,
}

impl Default for Project {

    fn default() -> Self {
        Self {
            folders: Default::default(),
            clips: Default::default(),
            palettes: Default::default()
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
    pub strokes: alisa::ObjList<Stroke>,
    pub layer_groups: alisa::ObjList<LayerGroup>,
    pub fills: alisa::ObjList<Fill>,
    pub palettes: alisa::ObjList<Palette>,
    pub palette_inners: alisa::ObjList<PaletteInner>,
    pub colors: alisa::ObjList<Color>
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
        alisa::ObjectKind::from::<LayerGroup>(),
        alisa::ObjectKind::from::<Fill>(),
        alisa::ObjectKind::from::<Palette>(),
        alisa::ObjectKind::from::<PaletteInner>(),
        alisa::ObjectKind::from::<Color>()
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
        alisa::OperationKind::from::<SetClipInnerBackgroundColor>(),
        
        alisa::OperationKind::from::<CreateLayer>(),
        alisa::OperationKind::from::<DeleteLayer>(),
        alisa::OperationKind::from::<TransferLayer>(),
        alisa::OperationKind::from::<SetLayerName>(),

        alisa::OperationKind::from::<CreateLayerGroup>(),
        alisa::OperationKind::from::<DeleteLayerGroup>(),
        alisa::OperationKind::from::<TransferLayerGroup>(),
        alisa::OperationKind::from::<SetLayerGroupName>(),

        alisa::OperationKind::from::<CreateFrame>(),
        alisa::OperationKind::from::<DeleteFrame>(),
        alisa::OperationKind::from::<SetFrameTime>(),

        alisa::OperationKind::from::<CreateStroke>(),
        alisa::OperationKind::from::<DeleteStroke>(),
        alisa::OperationKind::from::<SetStrokeStroke>(),
        alisa::OperationKind::from::<SetStrokeColor>(),

        alisa::OperationKind::from::<CreateFill>(),
        alisa::OperationKind::from::<DeleteFill>(),
        alisa::OperationKind::from::<SetFillPaths>(),
        alisa::OperationKind::from::<SetFillColor>(),

        alisa::OperationKind::from::<CreatePalette>(),
        alisa::OperationKind::from::<DeletePalette>(),
        alisa::OperationKind::from::<RenamePalette>(),
        alisa::OperationKind::from::<TransferPalette>(),

        alisa::OperationKind::from::<CreatePaletteInner>(),

        alisa::OperationKind::from::<CreateColor>(),
        alisa::OperationKind::from::<DeleteColor>(),
        alisa::OperationKind::from::<SetColorColor>(),
        alisa::OperationKind::from::<SetColorName>(),
    ];

}
