
use project::{Client, SceneObjPtr};
use crate::{get_brush_texture, get_color_value, BuiltinBrushTextures, EditorState, ScenePanel, SceneRenderList};

impl ScenePanel {
    
    pub(super) fn render_selection(rndr: &mut malvina::LayerRenderer, brushes: &BuiltinBrushTextures, client: &Client, editor: &EditorState, render_list: &SceneRenderList) {
        for scene_obj in render_list.objs.iter() {
            match scene_obj {
                SceneObjPtr::Stroke(stroke_ptr) => {
                    if !editor.selection.selected(*stroke_ptr) {
                        continue;
                    }
                    let Some(stroke) = editor.mesh_cache.get_stroke(*stroke_ptr) else { continue; };
                    let texture = get_brush_texture(stroke.brush, brushes);
                    rndr.render_stroke_selection(&stroke.mesh, get_color_value(&stroke.color, client), editor.scene_obj_transform(*stroke_ptr), Some(texture));
                },
                SceneObjPtr::Fill(fill_ptr) => {
                    if !editor.selection.selected(*fill_ptr) {
                        continue;
                    }
                    let Some(fill) = editor.mesh_cache.get_fill(*fill_ptr) else { continue; }; 
                    rndr.render_fill_selection(&fill.mesh, get_color_value(&fill.color, client), editor.scene_obj_transform(*fill_ptr));
                }
            }
        }
    }

}
