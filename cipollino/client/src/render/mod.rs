
use project::{Client, ClipInner, Fill, Frame, Layer, LayerPtr, Ptr, SceneObjPtr, Stroke};
use crate::{get_color_value, EditorState};

fn render_stroke(rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, stroke_ptr: Ptr<Stroke>) {
    if let Some(stroke) = editor.mesh_cache.get_stroke(stroke_ptr) {
        rndr.render_stroke(&stroke.mesh, get_color_value(&stroke.color, client), editor.scene_obj_transform(stroke_ptr));
    }
}

fn render_fill(rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, fill_ptr: Ptr<Fill>) {
    if let Some(fill) = editor.mesh_cache.get_fill(fill_ptr) {
        rndr.render_fill(&fill.mesh, get_color_value(&fill.color, client), editor.scene_obj_transform(fill_ptr)); 
    }
}

fn render_frame(rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, frame: &Frame) {
    for scene_child in frame.scene.iter().rev() {
        match scene_child {
            SceneObjPtr::Stroke(stroke_ptr) => {
                render_stroke(rndr, client, editor, stroke_ptr);
            },
            SceneObjPtr::Fill(fill_ptr) => {
                render_fill(rndr, client, editor, fill_ptr);
            }
        }
    }
}

fn render_layer(rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, layer: &Layer, layer_ptr: Ptr<Layer>, time: i32, editor_view: bool) {
    if editor_view && editor.hidden_layers.contains(&layer_ptr) {
        return;
    } 

    if layer_ptr == editor.active_layer && editor_view {
        if let Some(fill_preview) = &editor.preview.fill_preview {
            rndr.render_fill(fill_preview, get_color_value(&editor.color, client), elic::Mat4::IDENTITY);
        }
    }

    if let Some(frame_ptr) = layer.frame_at(client, time) {
        if let Some(frame) = client.get(frame_ptr) {
            render_frame(rndr, client, editor, frame);
        }
    } 

    if layer_ptr == editor.active_layer && editor_view {
        if let Some(stroke_preview) = &editor.preview.stroke_preview {
            rndr.render_stroke(stroke_preview, get_color_value(&editor.color, client), elic::Mat4::IDENTITY);
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
            },
            LayerPtr::LayerGroup(layer_ptr) => {
                if let Some(layer_group) = client.get(layer_ptr) {
                    render_layer_list(rndr, client, editor, &layer_group.layers, time, editor_view);
                }
            }
        } 
    }
}

pub fn render_scene(rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, clip: &ClipInner, time: i32, editor_view: bool) {
    render_layer_list(rndr, client, editor, &clip.layers, time, editor_view);
}
