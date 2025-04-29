use std::collections::HashSet;

use project::{Clip, Folder, Frame, Layer, Ptr, SceneObjPtr, Stroke};

use super::{Selectable, Selection, SelectionKind};


impl Selectable for Folder {
    const KIND: SelectionKind = SelectionKind::Asset;

    fn selection_list(selection: &Selection) -> &HashSet<Ptr<Self>> {
        &selection.folders
    }

    fn selection_list_mut(selection: &mut Selection) -> &mut HashSet<Ptr<Self>> {
        &mut selection.folders
    }
}

impl Selectable for Clip {
    const KIND: SelectionKind = SelectionKind::Asset;

    fn selection_list(selection: &Selection) -> &HashSet<Ptr<Self>> {
        &selection.clips
    }

    fn selection_list_mut(selection: &mut Selection) -> &mut HashSet<Ptr<Self>> {
        &mut selection.clips
    }
}

impl Selectable for Layer {
    const KIND: SelectionKind = SelectionKind::Layers;

    fn selection_list(selection: &Selection) -> &HashSet<Ptr<Self>> {
        &selection.layers
    }

    fn selection_list_mut(selection: &mut Selection) -> &mut HashSet<Ptr<Self>> {
        &mut selection.layers
    }
}

impl Selectable for Frame {
    const KIND: SelectionKind = SelectionKind::Frames;

    fn selection_list(selection: &Selection) -> &HashSet<Ptr<Self>> {
        &selection.frames
    }

    fn selection_list_mut(selection: &mut Selection) -> &mut HashSet<Ptr<Self>> {
        &mut selection.frames
    }
}

impl Selectable for Stroke {
    const KIND: SelectionKind = SelectionKind::Scene;

    fn selection_list(selection: &Selection) -> &HashSet<Ptr<Self>> {
        &selection.strokes
    }

    fn selection_list_mut(selection: &mut Selection) -> &mut HashSet<Ptr<Self>> {
        &mut selection.strokes
    }
}

impl Selection {

    pub fn is_scene_obj_selected(&self, obj: SceneObjPtr) -> bool {
        match obj {
            SceneObjPtr::Stroke(stroke) => self.selected(stroke),
        }
    } 

}
