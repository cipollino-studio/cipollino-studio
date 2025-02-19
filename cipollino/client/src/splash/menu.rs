
use std::path::Path;

use crate::{AppState, AppSystems, Editor};

use super::{recents::{add_recent, Recents}, CollabScreen, NewProjectScreen, SplashScreenState};

fn cleanup_recent_path(path: &Path, systems: &mut AppSystems) -> String {
    let path = pathdiff::diff_paths(path, &systems.home_path).unwrap_or(path.to_owned());
    path.to_string_lossy().into_owned()
}

pub(super) fn menu(ui: &mut pierro::UI, next_state: &mut Option<SplashScreenState>, next_app_state: &mut Option<AppState>, systems: &mut AppSystems) {

    let texture = pierro::include_image!(ui, "../../res/banner.png");
    pierro::scaled_image(ui, 0.4, texture);
    pierro::h_line(ui);

    pierro::margin(ui, pierro::Margin::same(10.0), |ui| {
        pierro::horizontal(ui, |ui| {
            pierro::container(ui, pierro::Size::fit(), pierro::Size::fr(1.0), pierro::Layout::vertical(), |ui| {

                // Empty label to vertically align with the "Recents" in the other column
                pierro::label(ui, "");
                pierro::v_spacing(ui, 5.0);

                if pierro::link_with_icon(ui, "New Project", pierro::icons::PLUS).mouse_clicked() {
                    *next_state = Some(SplashScreenState::NewProject(NewProjectScreen::default()));
                }
                pierro::v_spacing(ui, 3.0);

                if pierro::link_with_icon(ui, "Open Project", pierro::icons::FOLDER).mouse_clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Cipollino Project", &["cip"])
                        .pick_file() {
                            if let Some(editor) = Editor::local(path.clone()) {
                                add_recent(&mut systems.prefs, path);
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

            pierro::container(ui, pierro::Size::fr(1.0).with_grow(1.0), pierro::Size::fr(1.0), pierro::Layout::vertical(), |ui| {
                pierro::label(ui, "Recent Projects");
                pierro::v_spacing(ui, 5.0);

                for recent in systems.prefs.get::<Recents>() {
                    if pierro::link(ui, cleanup_recent_path(&recent, systems)).mouse_clicked() {
                        add_recent(&mut systems.prefs, recent.clone());
                        if let Some(editor) = Editor::local(recent) {
                            *next_app_state = Some(AppState::Editor(editor));
                        }
                    }
                    pierro::v_spacing(ui, 3.0);
                }
            });
        });
    });
}
