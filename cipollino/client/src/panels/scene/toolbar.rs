
use super::ScenePanel;

impl ScenePanel {

    pub(super) fn toolbar(&mut self, ui: &mut pierro::UI) {
        let bg = ui.style::<pierro::theme::BgDark>();
        let gap = 5.0;
        let margin = pierro::Margin::same(gap);
        ui.with_style::<pierro::theme::WidgetMargin, _, _>(pierro::Margin::same(3.0), |ui| {
            ui.with_style::<pierro::theme::LabelFontSize, _, _>(20.0, |ui| {
                ui.with_node(
                    pierro::UINodeParams::new(pierro::Size::fit(), pierro::Size::fr(1.0))
                        .with_fill(bg)
                        .with_margin(margin),
                    |ui| {
                        if pierro::icon_button(ui, pierro::icons::CURSOR).mouse_clicked() {

                        } 
                        pierro::v_spacing(ui, gap);

                        if pierro::icon_button(ui, pierro::icons::PENCIL).mouse_clicked() {

                        }
                });
            });
        });
        
    }

}
