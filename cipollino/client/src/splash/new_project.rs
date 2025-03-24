
use std::path::{Path, PathBuf};

use crate::{AppState, AppSystems, Editor};
use super::{recents::add_recent, SplashScreenState};

pub(super) struct NewProjectScreen {
    project_name: String,
    project_location: String,
}

fn get_path_relative_to_home_dir(path: &Path) -> PathBuf {
    directories::UserDirs::new()
        .map(|user_dirs| user_dirs.home_dir().to_owned())
        .and_then(|home_dir| pathdiff::diff_paths(path, home_dir))
        .unwrap_or(path.to_owned())
}

fn get_default_project_location() -> Option<String> {
    let user_dirs = directories::UserDirs::new()?;
    let document_dir = user_dirs.document_dir()?;
    let document_path = get_path_relative_to_home_dir(document_dir); 
    let project_path = document_path.join("Cipollino Projects/");
    let full_path = user_dirs.home_dir().join(project_path.clone());
    if !full_path.exists() {
        let _ = std::fs::create_dir_all(&full_path);
    }
    Some(project_path.to_string_lossy().into())
}

impl Default for NewProjectScreen {

    fn default() -> Self {
        Self {
            project_name: "Project".to_owned(),
            project_location: get_default_project_location().unwrap_or(String::new()), 
        }
    }

}

enum NewProjectErrorSource {
    Name,
    Location
}

impl NewProjectScreen {

    fn get_new_project_path(&self) -> Result<PathBuf, (&'static str, NewProjectErrorSource)> {
        let home_dir = directories::UserDirs::new().map(|user_dirs| user_dirs.home_dir().to_owned()).unwrap_or(PathBuf::new());
        let path = home_dir.join(PathBuf::from(&self.project_location));
        if !path.exists() {
            return Err(("Project location does not exist.", NewProjectErrorSource::Location));
        }
        if !path.is_dir() {
            return Err(("Project location must be a folder.", NewProjectErrorSource::Location));
        }
        if self.project_name.is_empty() {
            return Err(("Project name cannot be empty.", NewProjectErrorSource::Name));
        } 
        let project_name_path = PathBuf::from(&self.project_name).with_extension("cip");
        if project_name_path.components().count() > 1 {
            return Err(("Project name cannot be a path.", NewProjectErrorSource::Name));
        }
        let path = path.join(project_name_path);
        if path.exists() {
            return Err(("Project name taken.", NewProjectErrorSource::Name));
        }
        Ok(path)
    }

    pub fn render(&mut self, ui: &mut pierro::UI, next_state: &mut Option<SplashScreenState>, next_app_state: &mut Option<AppState>, error: &mut Option<String>, systems: &mut AppSystems) {

        // Back button
        if pierro::clickable_icon(ui, pierro::icons::ARROW_LEFT).mouse_clicked() {
            *next_state = Some(SplashScreenState::Menu);
        }
        pierro::v_spacing(ui, 5.0);

        pierro::vertical_centered(ui, |ui| {

            let (name_resp, location_resp) = pierro::key_value_layout(ui, |builder| {
                let name_resp = builder.labeled("Name:", |ui| {
                    pierro::text_edit(ui, &mut self.project_name)
                });
                let location_resp = builder.labeled("Location:", |ui| {
                    let location_resp = pierro::text_edit(ui, &mut self.project_location);
                    if pierro::icon_button(ui, pierro::icons::FOLDER).mouse_clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.project_location = get_path_relative_to_home_dir(&path).to_string_lossy().to_string();
                        }
                    }
                    location_resp
                });
                (name_resp, location_resp)
            });

            pierro::v_spacing(ui, 5.0);
            match self.get_new_project_path() {
                Ok(path) => {
                    if pierro::button(ui, "Create").mouse_clicked() {
                        if let Some(editor) = Editor::local(path.clone(), systems) {
                            add_recent(&mut systems.prefs, path);
                            *next_app_state = Some(AppState::Editor(editor));
                        } else {
                            *error = Some(format!("Could not create project at \"{}\".", path.canonicalize().unwrap_or(path).to_string_lossy().to_string()));
                        }
                    }
                },
                Err((error, source)) => {
                    let widget_margin = ui.style::<pierro::theme::WidgetMargin>();
                    pierro::margin(ui, widget_margin, |ui| {
                        pierro::error_label(ui, error);
                    });
                    match source {
                        NewProjectErrorSource::Name => pierro::error_outline(ui, name_resp.response.node_ref),
                        NewProjectErrorSource::Location => pierro::error_outline(ui, location_resp.response.node_ref),
                    }
                },
            }

        });

    }

}
