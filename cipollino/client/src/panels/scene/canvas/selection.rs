
use project::SceneChildPtr;
use crate::{EditorState, ScenePanel};

impl ScenePanel {
    
    pub(super) fn render_selection(&mut self, rndr: &mut malvina::LayerRenderer, editor: &EditorState, render_list: &Vec<SceneChildPtr>) {
        for scene_obj in render_list {
            match scene_obj {
                SceneChildPtr::Stroke(stroke_ptr) => {
                    if !editor.selection.selected(stroke_ptr.ptr()) {
                        continue;
                    }
                    if let Some(stroke) = editor.stroke_mesh_cache.get(&stroke_ptr.ptr()) {
                        rndr.render_stroke_selection(stroke, malvina::glam::vec4(0.0, 0.0, 0.0, 1.0));
                    }
                },
            }
        }
    }

}
