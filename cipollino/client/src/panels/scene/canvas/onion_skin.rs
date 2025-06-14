
use project::{Client, ClipInner, SceneObjPtr};

use crate::{AppSystems, EditorState, OnionSkinFutureColor, OnionSkinPastColor, ScenePanel, SceneRenderList};

impl ScenePanel {

    fn render_onion_skin_frame(rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, clip: &ClipInner, time: i32, color: elic::Color) {
        let render_list = SceneRenderList::make(client, editor, clip, time);
        for scene_obj in render_list.objs {
            match scene_obj {
                SceneObjPtr::Stroke(stroke_ptr) => {
                    if let Some(stroke) = editor.mesh_cache.get_stroke(stroke_ptr) {
                        rndr.render_stroke(&stroke.mesh, color, editor.scene_obj_transform(stroke_ptr), None);
                    }
                },
                SceneObjPtr::Fill(_fill_ptr) => {} // Fills shouldn't be rendered in the onion skin
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
