
use project::SceneObjPtr;

use crate::{EditorState, ScenePanel, SceneRenderList};

impl ScenePanel {

    fn render_picking_list(&mut self, rndr: &mut malvina::PickingRenderer, editor: &EditorState, render_list: &SceneRenderList, selected: bool) {
        for (idx, scene_obj) in render_list.objs.iter().enumerate() {
            if editor.selection.is_scene_obj_selected(*scene_obj) != selected {
                continue;
            }
            match scene_obj {
                SceneObjPtr::Stroke(stroke_ptr) => {
                    let stroke_mesh_cache = editor.stroke_mesh_cache.borrow();
                    if let Some(stroke) = stroke_mesh_cache.get(&stroke_ptr) {
                        rndr.render_stroke(stroke, idx as u32 + 1, editor.scene_obj_transform(*stroke_ptr));
                    }
                },
                SceneObjPtr::Fill(fill_ptr) => {
                    let fill_mesh_cache = editor.fill_mesh_cache.borrow();
                    if let Some(fill) = fill_mesh_cache.get(fill_ptr) {
                        rndr.render_fill(fill, idx as u32 + 1, editor.scene_obj_transform(*fill_ptr));
                    }
                }
            }
        }
    }

    pub(super) fn render_picking(&mut self, rndr: &mut malvina::PickingRenderer, editor: &EditorState, render_list: &SceneRenderList) {
        // First render things things that aren't selected...
        self.render_picking_list(rndr, editor, render_list, false); 
        // Then things that are
        self.render_picking_list(rndr, editor, render_list, true); 
        // This way, selected objects are "prioritized" in the picking buffer
    }

}
