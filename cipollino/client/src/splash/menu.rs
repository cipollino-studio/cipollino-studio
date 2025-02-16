
use crate::{AppState, Editor};

use super::{CollabScreen, SplashScreenState};

pub(super) fn menu(ui: &mut pierro::UI, next_state: &mut Option<SplashScreenState>, next_app_state: &mut Option<AppState>) {
    if pierro::button(ui, "Open").mouse_clicked() {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Cipollino Project", &["cip"])
            .pick_file() {
                if let Some(editor) = Editor::local(path) {
                    *next_app_state = Some(AppState::Editor(editor));
                }
        }
    }
    if pierro::button(ui, "Collab").mouse_clicked() {
        *next_state = Some(SplashScreenState::Collab(CollabScreen::new()));
    }
}
