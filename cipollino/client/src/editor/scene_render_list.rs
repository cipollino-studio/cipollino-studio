
use std::collections::HashSet;

use project::{Client, ClipInner, Frame, Layer, LayerPtr, Ptr, SceneObjPtr, Stroke};

use super::EditorState;

pub struct SceneRenderList {
    pub objs: Vec<SceneObjPtr>
}

impl SceneRenderList {

    fn get_frame_render_list(&mut self, frame: &Frame) {
        self.objs.extend(frame.scene.iter().rev());
    }

    fn get_layer_render_list(&mut self, client: &Client, editor: &EditorState, layer: &Layer, layer_ptr: Ptr<Layer>, time: i32) {
        if editor.hidden_layers.contains(&layer_ptr) {
            return;
        }

        let Some(frame_ptr) = layer.frame_at(client, time) else { return; };
        if let Some(frame) = client.get(frame_ptr) {
            self.get_frame_render_list(frame);
        }
    }

    fn get_layer_list_render_list(&mut self, client: &Client, editor: &EditorState, layer_list: &alisa::ChildList<LayerPtr>, time: i32) {
        for layer in layer_list.iter().rev() {
            match layer {
                LayerPtr::Layer(layer_ptr) => {
                    if let Some(layer) = client.get(layer_ptr) {
                        self.get_layer_render_list(client, editor, layer, layer_ptr, time);
                    }
                }
            } 
        }
    }

    pub fn make(client: &Client, editor: &EditorState, clip: &ClipInner, time: i32) -> Self {
        let mut list = SceneRenderList {
            objs: Vec::new(),
        };
        list.get_layer_list_render_list(client, editor, &clip.layers, time);
        list
    }

    pub fn rendered_strokes(&self) -> HashSet<Ptr<Stroke>> {
        let mut rendered_strokes = HashSet::new();
        for scene_obj in &self.objs {
            match scene_obj {
                SceneObjPtr::Stroke(ptr) => rendered_strokes.insert(*ptr),
            };
        }
        rendered_strokes
    }

}
