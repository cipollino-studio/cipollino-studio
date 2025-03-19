
use project::{Client, ClipInner, Frame, Layer, LayerChildList, LayerChildPtr, Ptr, SceneChildPtr, Stroke};
use crate::EditorState;

fn render_stroke(rndr: &mut malvina::LayerRenderer, editor: &EditorState, stroke: &Stroke, stroke_ptr: Ptr<Stroke>) {
    let mut stroke_mesh_cache = editor.stroke_mesh_cache.borrow_mut();
    if let Some(mesh) = stroke_mesh_cache.get(&stroke_ptr) {
        rndr.render_stroke(mesh, stroke.color.into());
    } else {
        let mesh = malvina::StrokeMesh::new(rndr.device(), &stroke.stroke.0);
        rndr.render_stroke(&mesh, stroke.color.into());
        stroke_mesh_cache.insert(stroke_ptr, mesh);
    }
}

fn render_frame(rndr: &mut malvina::LayerRenderer, editor: &EditorState, client: &Client, frame: &Frame) {
    for scene_child in frame.scene.iter().rev() {
        match scene_child {
            SceneChildPtr::Stroke(stroke_ptr) => {
                if let Some(stroke) = client.get(stroke_ptr.ptr()) {
                    render_stroke(rndr, editor, stroke, stroke_ptr.ptr());
                }
            }
        }
    }
}

fn render_layer(rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, layer: &Layer, layer_ptr: Ptr<Layer>, time: i32) {
    let Some(frame_ptr) = layer.frame_at(client, time) else { return; };
    if let Some(frame) = client.get(frame_ptr) {
        render_frame(rndr, editor, client, frame);
    }

    if layer_ptr == editor.active_layer {
        if let Some(stroke_preview) = &editor.stroke_preview {
            rndr.render_stroke(stroke_preview, malvina::glam::vec4(editor.color.r, editor.color.g, editor.color.b, editor.color.a));
        }
    } 
}

fn render_layer_list(rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, layer_list: &LayerChildList, time: i32) {
    for layer in layer_list.iter().rev() {
        match layer {
            LayerChildPtr::Layer(layer_ptr) => {
                if let Some(layer) = client.get(layer_ptr.ptr()) {
                    render_layer(rndr, client, editor, layer, layer_ptr.ptr(), time);
                }
            }
        } 
    }
}

pub fn render_scene(rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, clip: &ClipInner, time: i32) {
    render_layer_list(rndr, client, editor, &clip.layers, time);
}
