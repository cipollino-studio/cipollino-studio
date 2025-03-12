
use std::collections::HashSet;

use project::{Client, ClipInner, Frame, Layer, LayerChildList, LayerChildPtr, Ptr, SceneChildPtr, Stroke};
use crate::{EditorState, ScenePanel};

impl ScenePanel {

    fn get_frame_render_list(frame: &Frame, list: &mut Vec<SceneChildPtr>) {
        list.extend(frame.scene.iter().rev());
    }

    fn get_layer_render_list(client: &Client, editor: &EditorState, clip: &ClipInner, layer: &Layer, list: &mut Vec<SceneChildPtr>) {
        let Some(frame_ptr) = layer.frame_at(client, clip.frame_idx(editor.time)) else { return; };
        if let Some(frame) = client.get(frame_ptr) {
            Self::get_frame_render_list(frame, list);
        }
    }

    fn get_layer_list_render_list(client: &Client, editor: &EditorState, clip: &ClipInner, layer_list: &LayerChildList, list: &mut Vec<SceneChildPtr>) {
        for layer in layer_list.iter() {
            match layer {
                LayerChildPtr::Layer(layer_ptr) => {
                    if let Some(layer) = client.get(layer_ptr.ptr()) {
                        Self::get_layer_render_list(client, editor, clip, layer, list);
                    }
                }
            } 
        }
    }

    pub(super) fn render_list(client: &Client, editor: &EditorState, clip: &ClipInner) -> Vec<SceneChildPtr> {
        let mut list = Vec::new();
        Self::get_layer_list_render_list(client, editor, clip, &clip.layers, &mut list);
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
