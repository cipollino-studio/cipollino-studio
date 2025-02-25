
use project::Clip;

use super::TimelinePanel;

impl TimelinePanel {

    pub(super) const FRAMEBAR_HEIGHT: f32 = 20.0;
    pub(super) const FRAME_NUMBER_STEP: i32 = 5;

    const FRAME_NUMBER_NODE_WIDTH: f32 = Self::FRAME_WIDTH * (Self::FRAME_NUMBER_STEP as f32);

    fn frame_numbers(&mut self, ui: &mut pierro::UI, clip: &Clip) {
        pierro::h_spacing(ui, Self::FRAME_NUMBER_NODE_WIDTH / 2.0);
        for i in ((Self::FRAME_NUMBER_STEP - 1)..(clip.length as i32)).step_by(Self::FRAME_NUMBER_STEP as usize) {
            pierro::container(ui, pierro::Size::px(Self::FRAME_NUMBER_NODE_WIDTH), pierro::Size::fit(), pierro::Layout::vertical().align_center(), |ui| {
                pierro::label(ui, &(i + 1).to_string());
            });
        }
    }

    pub(super) fn framebar(&mut self, ui: &mut pierro::UI, clip: &Clip) -> pierro::ScrollAreaResponse<()> {
        let theme = ui.style::<pierro::Theme>();
        let fill = theme.bg_dark;
        ui.with_node(
            pierro::UINodeParams::new(pierro::Size::fr(1.0), pierro::Size::fit())
                .with_fill(fill),
            |ui| {
                let mut scroll_state = self.scroll_state;
                let response = pierro::ScrollArea::default()
                    .with_layout(pierro::Layout::horizontal().align_center())
                    .with_size(pierro::Size::fit(), pierro::Size::px(Self::FRAMEBAR_HEIGHT))
                    .hide_scroll_bars()
                    .with_state(&mut scroll_state)
                    .scroll_y(false)
                    .no_set_max_scroll()
                    .render(ui, |ui| {
                        self.frame_numbers(ui, clip);
                    });
                self.scroll_state = scroll_state;
                pierro::h_line(ui);
                response
        }).1
    }

}
