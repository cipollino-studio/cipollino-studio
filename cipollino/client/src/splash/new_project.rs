
use std::path::PathBuf;

use crate::{AppState, AppSystems, Editor};
use super::{recents::add_recent, SplashScreenState};


pub(super) struct NewProjectScreen {
    project_name: String,
    project_location: String,
}

fn get_default_project_location() -> String {
    if let Some(documents_path) = directories::UserDirs::new().and_then(|dirs| dirs.document_dir().map(|path| path.to_owned())) {
        let project_path = documents_path.join("Cipollino Projects/");
        if !project_path.exists() {
            let _ = std::fs::create_dir_all(&project_path);
        }
        return project_path.to_string_lossy().into();
    }
    String::new()
}

impl Default for NewProjectScreen {

    fn default() -> Self {
        Self {
            project_name: "Project".to_owned(),
            project_location: get_default_project_location(), 
        }
    }

}

fn labeled<F: FnOnce(&mut pierro::UI)>(ui: &mut pierro::UI, label: &str, contents: F) {
    pierro::horizontal_fit_centered(ui, |ui| {
        pierro::container(ui, pierro::Size::px(60.0), pierro::Size::fit(), pierro::Layout::horizontal().justify_max(), |ui| {
            pierro::label(ui, label);
        });

        pierro::h_spacing(ui, 4.0);

        contents(ui);
    });
}

impl NewProjectScreen {

    fn get_new_project_path(&self) -> Result<PathBuf, &'static str> {
        let path = PathBuf::from(&self.project_location);
        if !path.exists() {
            return Err("Project location does not exist.");
        }
        if !path.is_dir() {
            return Err("Project location must be a folder.");
        }
        let path = path.join(PathBuf::from(&self.project_name)).with_extension("cip");
        if path.exists() {
            return Err("Project name taken.");
        }
        Ok(path)
    }

    pub fn render(&mut self, ui: &mut pierro::UI, next_state: &mut Option<SplashScreenState>, next_app_state: &mut Option<AppState>, systems: &mut AppSystems) {
        pierro::margin(ui, pierro::Margin::same(10.0), |ui| {

            // Back button
            if pierro::clickable_icon(ui, pierro::icons::ARROW_LEFT).mouse_clicked() {
                *next_state = Some(SplashScreenState::Menu);
            }
            pierro::v_spacing(ui, 5.0);

            labeled(ui, "Name: ", |ui| {
                pierro::text_edit(ui, &mut self.project_name);
            });
            pierro::v_spacing(ui, 3.0);

            labeled(ui, "Location: ", |ui| {
                pierro::text_edit(ui, &mut self.project_location);
            });
            pierro::v_spacing(ui, 3.0);

            let project_path = match self.get_new_project_path() {
                Ok(path) => {
                    Some(path)
                },
                Err(error) => {
                    pierro::label(ui, error);
                    None
                },
            };
            pierro::v_spacing(ui, 15.0);

            if let Some(path) = project_path {
                pierro::vertical_centered(ui, |ui| {
                    if pierro::button(ui, "Create").mouse_clicked() {
                        if let Some(editor) = Editor::local(path.clone(), systems) {
                            add_recent(&mut systems.prefs, path);
                            *next_app_state = Some(AppState::Editor(editor));
                        }
                    }
                });
            }   
        });
    }

}
