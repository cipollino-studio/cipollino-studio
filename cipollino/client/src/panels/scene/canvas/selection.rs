
use project::{Client, SceneChildPtr};
use crate::{EditorState, ScenePanel};

impl ScenePanel {
    
    pub(super) fn render_selection(&mut self, rndr: &mut malvina::LayerRenderer, editor: &EditorState, client: &Client, render_list: &Vec<SceneChildPtr>) {
        for scene_obj in render_list {
            match scene_obj {
                SceneChildPtr::Stroke(stroke_ptr) => {
                    if !editor.selection.selected(stroke_ptr.ptr()) {
                        continue;
                    }
                    let Some(stroke) = client.get(stroke_ptr.ptr()) else { continue; };
                    let Some(stroke_mesh) = editor.stroke_mesh_cache.get(&stroke_ptr.ptr()) else { continue; };
                    rndr.render_stroke_selection(stroke_mesh, stroke.color.into());
                },
            }
        }
    }

}
