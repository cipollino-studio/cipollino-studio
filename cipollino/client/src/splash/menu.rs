
use std::path::{Path, PathBuf};

use crate::{AppState, AppSystems, Editor};

use super::{recents::{add_recent, remove_recent, Recents}, CollabScreen, NewProjectScreen, SplashScreenState};

fn cleanup_recent_path(path: &Path, systems: &mut AppSystems) -> String {
    let path = pathdiff::diff_paths(path, &systems.home_path).unwrap_or(path.to_owned());
    path.to_string_lossy().into_owned()
}

fn open_project(path: PathBuf, systems: &mut AppSystems, next_app_state: &mut Option<AppState>, error: &mut Option<String>) {
    if let Some(editor) = Editor::local(path.clone(), systems) {
        add_recent(&mut systems.prefs, path);
        *next_app_state = Some(AppState::Editor(editor));
    } else {
        *error = Some(format!("Could not open \"{}\".", path.to_string_lossy().to_string()));
    }
}

fn recent_context_menu(ui: &mut pierro::UI, response: &pierro::Response, systems: &mut AppSystems, recent: &PathBuf) {
    pierro::context_menu(ui, &response, |ui| {
        if pierro::menu_button(ui, "Remove from recents").mouse_clicked() {
            remove_recent(&mut systems.prefs, recent);
            pierro::close_context_menu(ui, response.id);
        }
    });
}

pub(super) fn menu(ui: &mut pierro::UI, next_state: &mut Option<SplashScreenState>, next_app_state: &mut Option<AppState>, error: &mut Option<String>, systems: &mut AppSystems) {

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
                        open_project(path, systems, next_app_state, error); 
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

            for recent in systems.prefs.get::<Recents>().into_iter().take(3) {
                let Some(file_name) = recent.file_stem() else { continue };
                let file_name = file_name.to_string_lossy().to_string();
                let cleaned_path = cleanup_recent_path(&recent, systems);

                ui.push_id_seed(&recent);
                let (response, _) = pierro::horizontal_fit(ui, |ui| {
                    if recent.exists() {
                        let link = pierro::link(ui, file_name);
                        pierro::weak_label(ui, format!(" ({})", cleaned_path));

                        if link.mouse_clicked() {
                            open_project(recent.clone(), systems, next_app_state, error); 
                        }
                        recent_context_menu(ui, &link, systems, &recent);
                    } else {
                        pierro::icon(ui, pierro::icons::WARNING);
                        pierro::h_spacing(ui, 2.0);
                        pierro::label(ui, file_name); 
                        pierro::weak_label(ui, format!(" ({})", cleaned_path));
                    }
                });

                recent_context_menu(ui, &response, systems, &recent);

                pierro::v_spacing(ui, 3.0);

            }
        });
    });

}
