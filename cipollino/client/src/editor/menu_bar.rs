
use crate::{splash::SplashScreen, AppState};

use super::{Editor, ExportDialog, SettingsWindow};

impl Editor {

    pub(super) fn menu_bar(&mut self, ui: &mut pierro::UI, next_app_state: &mut Option<AppState>) {

        pierro::menu_bar(ui, |ui| {
            pierro::menu_bar_item(ui, "File", |ui| {
                if pierro::menu_button(ui, "Export").mouse_clicked() {
                    self.state.editor.open_window(ExportDialog::new());
                }
                if self.state.project.client.is_collab() {
                    if pierro::menu_button(ui, "Disconnect").mouse_clicked() {
                        *next_app_state = Some(AppState::SplashScreen(SplashScreen::new()));
                    }
                }
            });
            pierro::menu_bar_item(ui, "Edit", |ui| {
                if pierro::menu_button(ui, "Undo").mouse_clicked() {
                    self.state.editor.will_undo = true;
                }
                if pierro::menu_button(ui, "Redo").mouse_clicked() {
                    self.state.editor.will_redo = true;
                }
                if pierro::menu_button(ui, "Settings...").mouse_clicked() {
                    self.state.editor.open_window(SettingsWindow::new());
                }
            });
        });
        
    }

}
