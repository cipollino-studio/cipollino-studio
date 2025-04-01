
use crate::PanelContext;

use super::Window;

mod appearance;
pub use appearance::*;

mod shortcuts;
use shortcuts::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum SettingsTab {
    Appearance,
    Shortcuts
}

pub struct SettingsWindow {
    tab: SettingsTab
}

impl SettingsWindow {

    pub fn new() -> Self {
        Self {
            tab: SettingsTab::Appearance
        }
    }

    fn settings_tab_button(&mut self, ui: &mut pierro::UI, label: &'static str, tab: SettingsTab) {
        ui.with_style::<pierro::theme::WidgetRounding, _, _>(pierro::Rounding::ZERO, |ui| {
            let bg = ui.style::<pierro::theme::BgDark>();
            let bg = if self.tab == tab {
                pierro::theme::pressed_color(bg)
            } else {
                bg
            };
            ui.with_style::<pierro::theme::BgButton, _, _>(bg, |ui| {
                let button = pierro::button(ui, label);
                ui.set_size(button.node_ref, pierro::Size::text().with_grow(1.0), pierro::Size::text());
                if button.mouse_clicked() {
                    self.tab = tab;
                }
            });
        });
    }

}

impl Window for SettingsWindow {

    fn title(&self) -> String {
        "Settings".to_owned()
    }

    fn render(&mut self, ui: &mut pierro::UI, _close: &mut bool, ctx: &mut PanelContext) {
        pierro::horizontal_fit(ui, |ui| {
            let bg = ui.style::<pierro::theme::BgDark>();
            ui.with_node(
                pierro::UINodeParams::new(pierro::Size::fit(), pierro::Size::fr(1.0))
                    .with_fill(bg),
                |ui| {
                    self.settings_tab_button(ui, "Appearance", SettingsTab::Appearance);
                    self.settings_tab_button(ui, "Shortcuts", SettingsTab::Shortcuts);
                }
            );
            pierro::v_line(ui);
            pierro::container(ui, pierro::Size::px(400.0), pierro::Size::px(300.0), pierro::Layout::vertical(), |ui| {
                pierro::scroll_area(ui, |ui| {
                    pierro::margin(ui, pierro::Margin::same(4.0), |ui| {
                        match self.tab {
                            SettingsTab::Appearance => appearance(ui, ctx.systems),
                            SettingsTab::Shortcuts => shortcuts(ui, ctx.systems),
                        }
                    });
                });
            });
        });
    }

    fn unique(&self) -> bool {
        true
    }

    fn use_margin(&self) -> bool {
        false
    }

}
