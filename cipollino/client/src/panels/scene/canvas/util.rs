
use std::collections::HashSet;

use project::{Client, ClipInner, Frame, Layer, LayerChildList, LayerChildPtr, Ptr, SceneChildPtr, Stroke};
use crate::{EditorState, ScenePanel};

impl ScenePanel {

    fn get_frame_render_list(frame: &Frame, list: &mut Vec<SceneChildPtr>) {
        list.extend(frame.scene.iter().rev());
    }

    fn get_layer_render_list(client: &Client, editor: &EditorState, layer: &Layer, layer_ptr: Ptr<Layer>, list: &mut Vec<SceneChildPtr>, time: i32) {
        if editor.hidden_layers.contains(&layer_ptr) {
            return;
        }

        let Some(frame_ptr) = layer.frame_at(client, time) else { return; };
        if let Some(frame) = client.get(frame_ptr) {
            Self::get_frame_render_list(frame, list);
        }
    }

    fn get_layer_list_render_list(client: &Client, editor: &EditorState, layer_list: &LayerChildList, list: &mut Vec<SceneChildPtr>, time: i32) {
        for layer in layer_list.iter().rev() {
            match layer {
                LayerChildPtr::Layer(layer_ptr) => {
                    if let Some(layer) = client.get(layer_ptr.ptr()) {
                        Self::get_layer_render_list(client, editor, layer, layer_ptr.ptr(), list, time);
                    }
                }
            } 
        }
    }

    pub(super) fn render_list(client: &Client, editor: &EditorState, clip: &ClipInner, time: i32) -> Vec<SceneChildPtr> {
        let mut list = Vec::new();
        Self::get_layer_list_render_list(client, editor, &clip.layers, &mut list, time);
        list
    }

    pub(super) fn rendered_strokes(render_list: &Vec<SceneChildPtr>) -> HashSet<Ptr<Stroke>> {
        let mut rendered_strokes = HashSet::new();
        for scene_obj in render_list {
            match scene_obj {
                SceneChildPtr::Stroke(ptr) => rendered_strokes.insert(ptr.ptr()),
            };
        }
        rendered_strokes
    }

}
