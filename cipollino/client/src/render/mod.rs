
use project::{Client, ClipInner, Frame, Layer, LayerPtr, Ptr, SceneChildPtr, Stroke};
use crate::EditorState;

fn render_stroke(rndr: &mut malvina::LayerRenderer, editor: &EditorState, stroke: &Stroke, stroke_ptr: Ptr<Stroke>) {
    let mut stroke_mesh_cache = editor.stroke_mesh_cache.borrow_mut();
    if let Some(mesh) = stroke_mesh_cache.get(&stroke_ptr) {
        rndr.render_stroke(mesh, stroke.color.into(), editor.stroke_transform(stroke_ptr));
    } else {
        let mesh = malvina::StrokeMesh::new(rndr.device(), &stroke.stroke.0, stroke.width);
        rndr.render_stroke(&mesh, stroke.color.into(), editor.stroke_transform(stroke_ptr));
        stroke_mesh_cache.insert(stroke_ptr, mesh);
    }
}

fn render_frame(rndr: &mut malvina::LayerRenderer, editor: &EditorState, client: &Client, frame: &Frame) {
    for scene_child in frame.scene.iter().rev() {
        match scene_child {
            SceneChildPtr::Stroke(stroke_ptr) => {
                if let Some(stroke) = client.get(stroke_ptr) {
                    render_stroke(rndr, editor, stroke, stroke_ptr);
                }
            }
        }
    }
}

fn render_layer(rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, layer: &Layer, layer_ptr: Ptr<Layer>, time: i32, editor_view: bool) {
    if editor_view && editor.hidden_layers.contains(&layer_ptr) {
        return;
    } 

    if let Some(frame_ptr) = layer.frame_at(client, time) {
        if let Some(frame) = client.get(frame_ptr) {
            render_frame(rndr, editor, client, frame);
        }
    } 

    if layer_ptr == editor.active_layer && editor_view {
        if let Some(stroke_preview) = &editor.preview.stroke_preview {
            rndr.render_stroke(stroke_preview, editor.color, elic::Mat4::IDENTITY);
        }
    } 
}

fn render_layer_list(rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, layer_list: &alisa::ChildList<LayerPtr>, time: i32, editor_view: bool) {
    for layer in layer_list.iter().rev() {
        match layer {
            LayerPtr::Layer(layer_ptr) => {
                if let Some(layer) = client.get(layer_ptr) {
                    render_layer(rndr, client, editor, layer, layer_ptr, time, editor_view);
                }
            }
        } 
    }
}

pub fn render_scene(rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, clip: &ClipInner, time: i32, editor_view: bool) {
    render_layer_list(rndr, client, editor, &clip.layers, time, editor_view);
}
