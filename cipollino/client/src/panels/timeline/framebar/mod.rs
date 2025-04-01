
use project::ClipInner;

use crate::{AppSystems, EditorState};

use super::TimelinePanel;

mod interaction;
mod overlay;
mod frame_numbers;

#[derive(Clone, Copy, PartialEq, Eq)]
enum DragTarget {
    PlayHead,
    OnionSkinPrev,
    OnionSkinNext
}

pub(super) struct Framebar {
    drag_target: DragTarget
}

impl TimelinePanel {

    pub(super) const FRAMEBAR_HEIGHT: f32 = 20.0;
    pub(super) const FRAME_NUMBER_STEP: i32 = 5;

    const FRAME_NUMBER_NODE_WIDTH: f32 = Self::FRAME_WIDTH * (Self::FRAME_NUMBER_STEP as f32);

}

impl Framebar {

    pub fn new() -> Self {
        Self {
            drag_target: DragTarget::PlayHead
        }
    }

    pub(super) fn render(&mut self, ui: &mut pierro::UI, editor: &mut EditorState, systems: &mut AppSystems, clip: &ClipInner, n_frames: u32, timeline_scroll_state: &mut pierro::ScrollAreaState) -> pierro::ScrollAreaResponse<pierro::Response> {
        let fill = ui.style::<pierro::theme::BgDark>();
        let framebar_scroll_response = ui.with_node(
            pierro::UINodeParams::new(pierro::Size::fr(1.0), pierro::Size::fit())
                .with_fill(fill),
            |ui| {
                let mut scroll_state = *timeline_scroll_state;
                let response = pierro::ScrollArea::default()
                    .with_size(pierro::Size::fit(), pierro::Size::px(TimelinePanel::FRAMEBAR_HEIGHT))
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
                *timeline_scroll_state = scroll_state;
                response
        }).1;

        let framebar_response = framebar_scroll_response.inner;
        self.mouse_interaction(ui, &framebar_response, editor, clip); 

        self.overlay(ui, editor, systems, clip, &framebar_response);
        
        framebar_scroll_response
    }

}
