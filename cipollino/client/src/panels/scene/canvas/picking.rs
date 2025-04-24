
use project::SceneChildPtr;

use crate::{EditorState, ScenePanel};

impl ScenePanel {

    pub(super) fn render_picking(&mut self, rndr: &mut malvina::PickingRenderer, editor: &EditorState, render_list: &Vec<SceneChildPtr>) {
        for scene_obj in render_list {
            match scene_obj {
                SceneChildPtr::Stroke(stroke_ptr) => {
                    let stroke_mesh_cache = editor.stroke_mesh_cache.borrow();
                    if let Some(stroke) = stroke_mesh_cache.get(&stroke_ptr) {
                        rndr.render_stroke(stroke, stroke_ptr.key() as u32, editor.stroke_transform(*stroke_ptr));
                    }
                },
            }
        }
    }

}
