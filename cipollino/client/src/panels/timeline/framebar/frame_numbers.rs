use crate::TimelinePanel;

use super::Framebar;


impl Framebar {

    pub(super) fn frame_numbers(&mut self, ui: &mut pierro::UI, n_frames: u32) {
        pierro::horizontal_fill_centered(ui, |ui| {

            let initial_spacing = ((TimelinePanel::FRAME_NUMBER_STEP / 2) as f32) * TimelinePanel::FRAME_WIDTH - if TimelinePanel::FRAME_NUMBER_STEP % 2 == 0 {
                0.5 * TimelinePanel::FRAME_WIDTH
            } else {
                0.0
            };
            pierro::h_spacing(ui, initial_spacing); 
            for i in ((TimelinePanel::FRAME_NUMBER_STEP - 1)..(n_frames as i32)).step_by(TimelinePanel::FRAME_NUMBER_STEP as usize) {
                pierro::container(ui, pierro::Size::px(TimelinePanel::FRAME_NUMBER_NODE_WIDTH), pierro::Size::fit(), pierro::Layout::vertical().align_center(), |ui| {
                    pierro::label(ui, &(i + 1).to_string());
                });
            }

            // Extend the framebar beyond the visible edges of the timeline panel
            pierro::h_spacing(ui, 100.0);

        });
    }

}
