
use project::SceneObjPtr;

use crate::{EditorState, ScenePanel};

impl ScenePanel {

    pub(super) fn render_picking(&mut self, rndr: &mut malvina::PickingRenderer, editor: &EditorState, render_list: &Vec<SceneObjPtr>) {
        for (idx, scene_obj) in render_list.iter().enumerate() {
            match scene_obj {
                SceneObjPtr::Stroke(stroke_ptr) => {
                    let stroke_mesh_cache = editor.stroke_mesh_cache.borrow();
                    if let Some(stroke) = stroke_mesh_cache.get(&stroke_ptr) {
                        rndr.render_stroke(stroke, idx as u32 + 1, editor.stroke_transform(*stroke_ptr));
                    }
                },
            }
        }
    }

}
