
use project::Clip;

use super::{RenderList, TimelinePanel};

impl TimelinePanel {

    pub(super) const FRAME_WIDTH: f32 = 12.0;
    pub(super) const LAYER_HEIGHT: f32 = 20.0; 

    pub(super) fn frame_area(&mut self, ui: &mut pierro::UI, render_list: &RenderList, clip: &Clip) -> pierro::ScrollAreaResponse<()> {
        let mut scroll_state = self.scroll_state;
        let response = pierro::ScrollArea::default()
            .with_state(&mut scroll_state)
            .render(ui, |ui| {
                pierro::container(ui, pierro::Size::px((clip.length as f32) * Self::FRAME_WIDTH), pierro::Size::px((render_list.len() as f32) * Self::LAYER_HEIGHT), pierro::Layout::horizontal(), |ui| {
                    pierro::label(ui, "Hey hey"); 
                });
            });
        self.scroll_state = scroll_state;
        response
    }

}
