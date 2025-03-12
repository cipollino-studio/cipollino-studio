
use crate::{EditorState, ScenePanel};

use project::{Client, ClipInner, Frame, Layer, LayerChildList, LayerChildPtr, Ptr, SceneChildPtr, Stroke};

impl ScenePanel {

    fn render_stroke(&mut self, rndr: &mut malvina::LayerRenderer, editor: &EditorState, stroke: &Stroke, stroke_ptr: Ptr<Stroke>) {
        if let Some(mesh) = editor.stroke_mesh_cache.get(&stroke_ptr) {
            rndr.render_stroke(mesh, stroke.color.into());
        }
    }

    fn render_frame(&mut self, rndr: &mut malvina::LayerRenderer, editor: &EditorState, client: &Client, frame: &Frame) {
        for scene_child in frame.scene.iter().rev() {
            match scene_child {
                SceneChildPtr::Stroke(stroke_ptr) => {
                    if let Some(stroke) = client.get(stroke_ptr.ptr()) {
                        self.render_stroke(rndr, editor, stroke, stroke_ptr.ptr());
                    }
                }
            }
        }
    }

    fn render_layer(&mut self, rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, clip: &ClipInner, layer: &Layer, layer_ptr: Ptr<Layer>) {
        let Some(frame_ptr) = layer.frame_at(client, clip.frame_idx(editor.time)) else { return; };
        if let Some(frame) = client.get(frame_ptr) {
            self.render_frame(rndr, editor, client, frame);
        }

        if layer_ptr == editor.active_layer {
            if let Some(stroke_preview) = &editor.stroke_preview {
                rndr.render_stroke(stroke_preview, malvina::glam::vec4(editor.color.r, editor.color.g, editor.color.b, editor.color.a));
            }
        } 
    }

    pub(super) fn render_layer_list(&mut self, rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, clip: &ClipInner, layer_list: &LayerChildList) {
        for layer in layer_list.iter().rev() {
            match layer {
                LayerChildPtr::Layer(layer_ptr) => {
                    if let Some(layer) = client.get(layer_ptr.ptr()) {
                        self.render_layer(rndr, client, editor, clip, layer, layer_ptr.ptr());
                    }
                }
            } 
        }
    }

}