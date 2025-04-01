
use project::ClipInner;

use crate::{AppSystems, EditorState, OnionSkinFutureColor, OnionSkinPastColor, TimelinePanel};

use super::Framebar;

impl Framebar {

    pub(super) fn overlay(&mut self, ui: &mut pierro::UI, editor: &mut EditorState, systems: &mut AppSystems, clip: &ClipInner, framebar_response: &pierro::Response) {
        let curr_frame = clip.frame_idx(editor.time);
        let show_onion_skin = editor.show_onion_skin;
        let onion_skin_prev_frames = editor.onion_skin_prev_frames as i32;
        let onion_skin_next_frames = editor.onion_skin_next_frames as i32;
        let accent_color = ui.style::<pierro::theme::AccentColor>();
        let past_onion_skin_color = systems.prefs.get::<OnionSkinPastColor>();
        let future_onion_skin_color = systems.prefs.get::<OnionSkinFutureColor>();
        ui.set_on_paint(framebar_response.node_ref, move |painter, rect| {

            let frame_rect = |frame: i32| pierro::Rect::min_size(
                rect.tl() + pierro::Vec2::X * (frame as f32) * TimelinePanel::FRAME_WIDTH,
                pierro::vec2(TimelinePanel::FRAME_WIDTH, rect.height())
            );

            painter.rect(
                pierro::PaintRect::new(frame_rect(curr_frame), accent_color.with_alpha(0.2))
                    .with_stroke(pierro::Stroke::new(accent_color, 1.5))
                    .with_rounding(pierro::Rounding::same(3.0))
            );

            if show_onion_skin {

                let left_onion_skin_rect = frame_rect((curr_frame - onion_skin_prev_frames).max(0));
                painter.with_clip_rect(left_onion_skin_rect.left_half(), |painter| {
                    painter.rect(
                        pierro::PaintRect::new(left_onion_skin_rect, pierro::Color::TRANSPARENT)
                            .with_stroke(pierro::Stroke::new(past_onion_skin_color, 1.5))
                            .with_rounding(pierro::Rounding::same(3.0))
                    );
                });

                let right_onion_skin_rect = frame_rect((curr_frame + onion_skin_next_frames).max(0));
                painter.with_clip_rect(right_onion_skin_rect.right_half(), |painter| {
                    painter.rect(
                        pierro::PaintRect::new(right_onion_skin_rect, pierro::Color::TRANSPARENT)
                            .with_stroke(pierro::Stroke::new(future_onion_skin_color, 1.5))
                            .with_rounding(pierro::Rounding::same(3.0))
                    );
                });
            }

        });
    }

}
