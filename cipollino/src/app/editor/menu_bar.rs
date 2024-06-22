
use crate::{app::{splash_screen::SplashScreen, AppState, AppSystems}, panels::PANEL_KINDS};

use super::{keybind::{RedoKeybind, UndoKeybind}, Editor};

impl Editor {

    pub fn menu_bar(&mut self, ctx: &egui::Context, next_state: &mut Option<AppState>, disabled: bool, systems: &mut AppSystems) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.set_enabled(!disabled);
            egui::menu::bar(ui, |ui| {

                #[cfg(not(target_arch = "wasm32"))]
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        ui.close_menu();
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.add_enabled(
                        self.state.actions.can_undo(),
                        egui::Button::new("Undo").shortcut_text(ui.ctx().format_shortcut(&systems.prefs.get::<UndoKeybind>()))).clicked() {
                        self.state.actions.undo(&mut self.state.project, &mut self.state.client);
                    }
                    if ui.add_enabled(
                        self.state.actions.can_redo(),
                        egui::Button::new("Redo").shortcut_text(ui.ctx().format_shortcut(&systems.prefs.get::<RedoKeybind>()))).clicked() {
                        self.state.actions.redo(&mut self.state.project, &mut self.state.client);
                    }
                });

                if self.state.client.is_collab() {
                    ui.menu_button("Collab", |ui| {
                        if ui.button("Disconnect").clicked() {
                            *next_state = Some(AppState::SplashScreen(SplashScreen::default()));
                            ui.close_menu();
                        }
                    });
                }

                ui.menu_button("View", |ui| {
                    ui.menu_button("Add Panel", |ui| {
                        for panel_kind in PANEL_KINDS {
                            if ui.button(panel_kind.title()).clicked() {
                                self.panel_manager.add_panel(panel_kind);
                            }
                        }
                    });
                });

            });
        });
    }

}
