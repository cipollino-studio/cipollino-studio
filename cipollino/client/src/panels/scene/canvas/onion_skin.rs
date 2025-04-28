
use project::{Client, ClipInner};

use crate::{AppSystems, EditorState, OnionSkinFutureColor, OnionSkinPastColor, ScenePanel};

impl ScenePanel {

    fn render_onion_skin_frame(rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, clip: &ClipInner, time: i32, color: elic::Color) {
        let render_list = Self::render_list(client, editor, clip, time);
        for scene_obj in render_list {
            match scene_obj {
                project::SceneObjPtr::Stroke(stroke_ptr) => {
                    if let Some(stroke) = client.get(stroke_ptr) {
                        let mut stroke_mesh_cache = editor.stroke_mesh_cache.borrow_mut();
                        if let Some(stroke) = stroke_mesh_cache.get(&stroke_ptr) {
                            rndr.render_stroke(stroke, color, elic::Mat4::IDENTITY);
                        } else {
                            let mesh = malvina::StrokeMesh::new(rndr.device(), &stroke.stroke.0, stroke.width);
                            rndr.render_stroke(&mesh, color, elic::Mat4::IDENTITY);
                            stroke_mesh_cache.insert(stroke_ptr, mesh);
                        }
                    }
                },
            }
        }
    }

    pub(super) fn render_onion_skin(rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, systems: &mut AppSystems, clip: &ClipInner) {
        let curr_frame = clip.frame_idx(editor.time);

        let past_color = systems.prefs.get::<OnionSkinPastColor>();
        let future_color = systems.prefs.get::<OnionSkinFutureColor>();

        // Render prev onion skins
        for i in (1..=editor.onion_skin_prev_frames).rev() {
            let frame = curr_frame - (i as i32); 
            let alpha = 0.7 * 0.8f32.powi(i as i32);
            let color = elic::Color::WHITE.lerp(past_color, alpha);
            Self::render_onion_skin_frame(rndr, client, editor, clip, frame, color);
        }

        // Render next onion skins
        for i in (1..=editor.onion_skin_next_frames).rev() {
            let frame = curr_frame + (i as i32); 
            let alpha = 0.7 * 0.8f32.powi(i as i32);
            let color = elic::Color::WHITE.lerp(future_color, alpha);
            Self::render_onion_skin_frame(rndr, client, editor, clip, frame, color);
        }

    }

}
