
mod builtin_brushes;
pub use builtin_brushes::*;

use project::{Client, ClipInner, Fill, Frame, Layer, LayerPtr, Ptr, SceneObjPtr, Stroke};
use crate::{get_brush_texture, get_color_value, EditorState};

fn render_stroke(rndr: &mut malvina::LayerRenderer, brushes: &BuiltinBrushTextures, client: &Client, editor: &mut EditorState, stroke_ptr: Ptr<Stroke>) {
    if editor.mesh_cache.get_stroke(stroke_ptr).is_none() {
        editor.mesh_cache.calculate_stroke_mesh(stroke_ptr, client, rndr.device());
    }

    let Some(stroke_mesh) = editor.mesh_cache.get_stroke(stroke_ptr) else { return; };
    let texture = get_brush_texture(stroke_mesh.brush, brushes);
    rndr.render_stroke(&stroke_mesh.mesh, get_color_value(&stroke_mesh.color, client), editor.scene_obj_transform(stroke_ptr), Some(texture));
}

fn render_fill(rndr: &mut malvina::LayerRenderer, client: &Client, editor: &mut EditorState, fill_ptr: Ptr<Fill>) {
    if editor.mesh_cache.get_fill(fill_ptr).is_none() {
        editor.mesh_cache.calculate_fill_mesh(fill_ptr, client, rndr.device());
    }

    if let Some(fill) = editor.mesh_cache.get_fill(fill_ptr) {
        rndr.render_fill(&fill.mesh, get_color_value(&fill.color, client), editor.scene_obj_transform(fill_ptr)); 
    }
}

fn render_frame(rndr: &mut malvina::LayerRenderer, brushes: &BuiltinBrushTextures, client: &Client, editor: &mut EditorState, frame: &Frame, editor_view: bool) {
    for scene_child in frame.scene.iter().rev() {
        if editor_view && editor.preview.hide.contains(&scene_child) {
            continue;
        }
        match scene_child {
            SceneObjPtr::Stroke(stroke_ptr) => {
                render_stroke(rndr, brushes, client, editor, stroke_ptr);
            },
            SceneObjPtr::Fill(fill_ptr) => {
                render_fill(rndr, client, editor, fill_ptr);
            }
        }
    }
}

fn render_layer(rndr: &mut malvina::LayerRenderer, brushes: &BuiltinBrushTextures, client: &Client, editor: &mut EditorState, layer: &Layer, layer_ptr: Ptr<Layer>, time: i32, editor_view: bool) {
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
            render_frame(rndr, brushes, client, editor, frame, editor_view);
        }
    } 

    if layer_ptr == editor.active_layer && editor_view {
        if let Some(stroke_preview) = &editor.preview.stroke_preview {
            let texture = get_brush_texture(editor.brush, brushes);
            rndr.render_stroke(stroke_preview, get_color_value(&editor.color, client), elic::Mat4::IDENTITY, Some(texture));
        }
    }
}

fn render_layer_list(rndr: &mut malvina::LayerRenderer, brushes: &BuiltinBrushTextures, client: &Client, editor: &mut EditorState, layer_list: &alisa::ChildList<LayerPtr>, time: i32, editor_view: bool) {
    for layer in layer_list.iter().rev() {
        match layer {
            LayerPtr::Layer(layer_ptr) => {
                if let Some(layer) = client.get(layer_ptr) {
                    render_layer(rndr, brushes, client, editor, layer, layer_ptr, time, editor_view);
                }
            },
            LayerPtr::LayerGroup(layer_ptr) => {
                if let Some(layer_group) = client.get(layer_ptr) {
                    render_layer_list(rndr, brushes, client, editor, &layer_group.layers, time, editor_view);
                }
            }
            LayerPtr::AudioLayer(_) => {},
        }
    }
}

pub fn render_scene(rndr: &mut malvina::LayerRenderer, brushes: &BuiltinBrushTextures, client: &Client, editor: &mut EditorState, clip: &ClipInner, time: i32, editor_view: bool) {
    render_layer_list(rndr, brushes, client, editor, &clip.layers, time, editor_view);
}
