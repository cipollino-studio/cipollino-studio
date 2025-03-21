
use project::{Client, ClipInner};

use crate::{EditorState, ScenePanel, ONION_SKIN_NEXT_COLOR, ONION_SKIN_PREV_COLOR};

impl ScenePanel {

    fn render_onion_skin_frame(&mut self, rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, clip: &ClipInner, time: i32, color: malvina::glam::Vec4) {
        let render_list = Self::render_list(client, clip, time);
        for scene_obj in render_list {
            match scene_obj {
                project::SceneChildPtr::Stroke(stroke_ptr) => {
                    if let Some(stroke) = client.get(stroke_ptr.ptr()) {
                        let mut stroke_mesh_cache = editor.stroke_mesh_cache.borrow_mut();
                        if let Some(stroke) = stroke_mesh_cache.get(&stroke_ptr.ptr()) {
                            rndr.render_stroke(stroke, color);
                        } else {
                            let mesh = malvina::StrokeMesh::new(rndr.device(), &stroke.stroke.0);
                            rndr.render_stroke(&mesh, color);
                            stroke_mesh_cache.insert(stroke_ptr.ptr(), mesh);
                        }
                    }
                },
            }
        }
    }

    pub(super) fn render_onion_skin(&mut self, rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, clip: &ClipInner) {
        let curr_frame = clip.frame_idx(editor.time);

        // Render prev onion skins
        let prev_onion_skin_color = malvina::glam::vec4(ONION_SKIN_PREV_COLOR.r, ONION_SKIN_PREV_COLOR.g, ONION_SKIN_PREV_COLOR.b, 1.0);
        for i in (1..=editor.onion_skin_prev_frames).rev() {
            let frame = curr_frame - (i as i32); 
            let alpha = 0.7 * 0.8f32.powi(i as i32);
            let color = malvina::glam::Vec4::ONE.lerp(prev_onion_skin_color, alpha);
            self.render_onion_skin_frame(rndr, client, editor, clip, frame, color);
        }

        // Render next onion skins
        let next_onion_skin_color = malvina::glam::vec4(ONION_SKIN_NEXT_COLOR.r, ONION_SKIN_NEXT_COLOR.g, ONION_SKIN_NEXT_COLOR.b, 1.0);
        for i in (1..=editor.onion_skin_next_frames).rev() {
            let frame = curr_frame + (i as i32); 
            let alpha = 0.7 * 0.8f32.powi(i as i32);
            let color = malvina::glam::Vec4::ONE.lerp(next_onion_skin_color, alpha);
            self.render_onion_skin_frame(rndr, client, editor, clip, frame, color);
        }

    }

}
