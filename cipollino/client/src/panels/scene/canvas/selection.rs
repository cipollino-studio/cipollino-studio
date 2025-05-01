
use project::{Client, SceneObjPtr};
use crate::{EditorState, ScenePanel, SceneRenderList};

impl ScenePanel {
    
    pub(super) fn render_selection(rndr: &mut malvina::LayerRenderer, editor: &EditorState, client: &Client, render_list: &SceneRenderList) {
        for scene_obj in render_list.objs.iter() {
            match scene_obj {
                SceneObjPtr::Stroke(stroke_ptr) => {
                    if !editor.selection.selected(*stroke_ptr) {
                        continue;
                    }
                    let Some(stroke) = client.get(*stroke_ptr) else { continue; };
                    let stroke_mesh_cache = editor.stroke_mesh_cache.borrow();
                    let Some(stroke_mesh) = stroke_mesh_cache.get(&stroke_ptr) else { continue; };
                    rndr.render_stroke_selection(stroke_mesh, stroke.color.into(), editor.scene_obj_transform(*stroke_ptr));
                },
                SceneObjPtr::Fill(fill_ptr) => {
                    if !editor.selection.selected(*fill_ptr) {
                        continue;
                    }
                    let Some(fill) = client.get(*fill_ptr) else { continue; };
                    let fill_mesh_cache = editor.fill_mesh_cache.borrow();
                    let Some(fill_mesh) = fill_mesh_cache.get(fill_ptr) else { continue; };
                    rndr.render_fill_selection(fill_mesh, fill.color.into(), editor.scene_obj_transform(*fill_ptr));
                }
            }
        }
    }

}
