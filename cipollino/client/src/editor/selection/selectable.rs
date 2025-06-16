use std::collections::HashSet;

use project::{AudioInstance, Clip, Fill, Folder, Frame, Layer, Ptr, SceneObjPtr, Stroke};

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

impl Selectable for AudioInstance {
    const KIND: SelectionKind = SelectionKind::Frames;

    fn selection_list(selection: &Selection) -> &HashSet<Ptr<Self>> {
        &selection.audio_instances
    }

    fn selection_list_mut(selection: &mut Selection) -> &mut HashSet<Ptr<Self>> {
        &mut selection.audio_instances
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

impl Selectable for Fill {
    const KIND: SelectionKind = SelectionKind::Scene;

    fn selection_list(selection: &Selection) -> &HashSet<Ptr<Self>> {
        &selection.fills
    }

    fn selection_list_mut(selection: &mut Selection) -> &mut HashSet<Ptr<Self>> {
        &mut selection.fills
    }
}

impl Selection {

    pub fn is_scene_obj_selected(&self, obj: SceneObjPtr) -> bool {
        match obj {
            SceneObjPtr::Stroke(stroke) => self.selected(stroke),
            SceneObjPtr::Fill(fill) => self.selected(fill),
        }
    } 

    pub fn select_scene_obj(&mut self, obj: SceneObjPtr) {
        match obj {
            SceneObjPtr::Stroke(ptr) => self.select(ptr),
            SceneObjPtr::Fill(ptr) => self.select(ptr),
        }
    }

    pub fn extend_select_scene_obj(&mut self, obj: SceneObjPtr) {
        match obj {
            SceneObjPtr::Stroke(ptr) => self.extend_select(ptr),
            SceneObjPtr::Fill(ptr) => self.extend_select(ptr),
        }
    }

}
