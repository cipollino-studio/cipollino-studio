
use crate::{AppState, Editor};

use super::{CollabScreen, NewProjectScreen, SplashScreenState};

pub(super) fn menu(ui: &mut pierro::UI, next_state: &mut Option<SplashScreenState>, next_app_state: &mut Option<AppState>) {

    let texture = pierro::include_image!(ui, "../../res/banner.png");
    pierro::scaled_image(ui, 0.4, texture);
    pierro::h_line(ui);

    pierro::margin(ui, pierro::Margin::same(10.0), |ui| {
        pierro::horizontal(ui, |ui| {
            pierro::container(ui, pierro::Size::fr(1.0), pierro::Size::fr(1.0), pierro::Layout::vertical(), |ui| {
                // Empty label to vertically align with the "Recents" in the other column
                pierro::label(ui, "");

                if pierro::link_with_icon(ui, "New Project", pierro::icons::PLUS).mouse_clicked() {
                    *next_state = Some(SplashScreenState::NewProject(NewProjectScreen::default()));
                }
                pierro::v_spacing(ui, 3.0);

                if pierro::link_with_icon(ui, "Open Project", pierro::icons::FOLDER).mouse_clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Cipollino Project", &["cip"])
                        .pick_file() {
                            if let Some(editor) = Editor::local(path) {
                                *next_app_state = Some(AppState::Editor(editor));
                            }
                    }
                }
                pierro::v_spacing(ui, 3.0);

                if pierro::link_with_icon(ui, "Collab", pierro::icons::CLOUD).mouse_clicked() {
                    *next_state = Some(SplashScreenState::Collab(CollabScreen::new()));
                }
            });

            pierro::h_spacing(ui, 10.0);

            pierro::container(ui, pierro::Size::fr(1.0), pierro::Size::fr(1.0), pierro::Layout::vertical().align_center(), |ui| {
                pierro::label(ui, "Recent Projects");
            });
        });
    });
}
