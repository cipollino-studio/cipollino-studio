
use project::ClipInner;

use crate::EditorState;

use super::TimelinePanel;

impl TimelinePanel {

    pub(super) const FRAMEBAR_HEIGHT: f32 = 20.0;
    pub(super) const FRAME_NUMBER_STEP: i32 = 5;

    const FRAME_NUMBER_NODE_WIDTH: f32 = Self::FRAME_WIDTH * (Self::FRAME_NUMBER_STEP as f32);

    fn frame_numbers(&mut self, ui: &mut pierro::UI, n_frames: u32) {
        pierro::horizontal_fill_centered(ui, |ui| {

            let initial_spacing = ((Self::FRAME_NUMBER_STEP / 2) as f32) * Self::FRAME_WIDTH - if Self::FRAME_NUMBER_STEP % 2 == 0 {
                0.5 * Self::FRAME_WIDTH
            } else {
                0.0
            };
            pierro::h_spacing(ui, initial_spacing); 
            for i in ((Self::FRAME_NUMBER_STEP - 1)..(n_frames as i32)).step_by(Self::FRAME_NUMBER_STEP as usize) {
                pierro::container(ui, pierro::Size::px(Self::FRAME_NUMBER_NODE_WIDTH), pierro::Size::fit(), pierro::Layout::vertical().align_center(), |ui| {
                    pierro::label(ui, &(i + 1).to_string());
                });
            }

            // Extend the framebar beyond the visible edges of the timeline panel
            pierro::h_spacing(ui, 100.0);

        });
    }

    pub(super) fn framebar(&mut self, ui: &mut pierro::UI, editor: &mut EditorState, clip: &ClipInner, n_frames: u32) -> pierro::ScrollAreaResponse<pierro::Response> {
        let fill = ui.style::<pierro::theme::BgDark>();
        let framebar_scroll_response = ui.with_node(
            pierro::UINodeParams::new(pierro::Size::fr(1.0), pierro::Size::fit())
                .with_fill(fill),
            |ui| {
                let mut scroll_state = self.scroll_state;
                let response = pierro::ScrollArea::default()
                    .with_size(pierro::Size::fit(), pierro::Size::px(Self::FRAMEBAR_HEIGHT))
                    .hide_scroll_bars()
                    .with_state(&mut scroll_state)
                    .scroll_y(false)
                    .no_set_max_scroll()
                    .render(ui, |ui| {
                        pierro::vertical_fill(ui, |ui| {
                            self.frame_numbers(ui, n_frames);
                            pierro::h_line(ui);
                        }).0
                    });
                self.scroll_state = scroll_state;
                response
        }).1;

        let framebar_response = framebar_scroll_response.inner;

        // Framebar mouse interactions
        ui.set_sense_mouse(framebar_response.node_ref, true);
        if framebar_response.drag_started() {
            framebar_response.request_focus(ui);
        }
        if framebar_response.is_focused(ui) && !framebar_response.dragging() {
            framebar_response.release_focus(ui);
        }
        if let Some(mouse_pos) = framebar_response.mouse_pos(ui) {
            let frame = ((mouse_pos.x / Self::FRAME_WIDTH).floor() as i32).min(clip.length as i32 - 1).max(0);
            if framebar_response.mouse_clicked() || framebar_response.is_focused(ui) {
                editor.jump_to((frame as f32) * clip.frame_len());
            }
        }

        // Draw the playback head
        let curr_frame = clip.frame_idx(editor.time);
        let accent_color = ui.style::<pierro::theme::AccentColor>();
        ui.set_on_paint(framebar_response.node_ref, move |painter, rect| {
            let playback_head_rect = pierro::Rect::min_size(
                rect.tl() + pierro::Vec2::X * (curr_frame as f32) * Self::FRAME_WIDTH,
                pierro::vec2(Self::FRAME_WIDTH, rect.height())
            );
            painter.rect(
                pierro::PaintRect::new(playback_head_rect, accent_color.with_alpha(0.2))
                    .with_stroke(pierro::Stroke::new(accent_color, 1.5))
                    .with_rounding(pierro::Rounding::same(3.0))
            );
        });

        framebar_scroll_response
    }

}
