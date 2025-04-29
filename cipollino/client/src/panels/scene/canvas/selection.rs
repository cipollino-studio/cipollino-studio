
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
                    rndr.render_stroke_selection(stroke_mesh, stroke.color.into(), editor.stroke_transform(*stroke_ptr));
                },
            }
        }
    }

}
