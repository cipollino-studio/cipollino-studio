
use std::path::PathBuf;

use cipollino_project::client::ProjectClient;

use crate::{editor::Editor, util::ui::{clickable_label, error_label, key_value_layout, key_value_row, path_selector}};

use super::{splash_screen::SplashScreen, util::centered_fixed_window, AppState, AppSystems};

pub struct NewProject {
    project_name: String,
    project_location: PathBuf,
    project_fps: f32,
    project_sample_rate: f32
}

const FPS_OPTIONS: [f32; 5] = [18.0, 24.0, 30.0, 48.0, 60.0];
const SAMPLE_RATE_OPTIONS: [f32; 5] = [44100.0, 48000.0, 88200.0, 96000.0, 192000.0];

pub fn default_project_location() -> PathBuf {
    // let user_dirs = directories::UserDirs::new().unwrap();
    // let mut default_project_location = user_dirs.document_dir().unwrap().to_owned();
    // default_project_location.push("Cipollino Projects");
    // default_project_location
    PathBuf::new()
}

impl Default for NewProject {

    fn default() -> Self {
        Self {
            project_name: "".to_owned(),
            project_location: default_project_location(),
            project_fps: 24.0,
            project_sample_rate: 44100.0
        }
    }


}

impl NewProject {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, ctx: &egui::Context, systems: &mut AppSystems) -> Option<AppState> {

        let mut next_state = None;

        egui::CentralPanel::default().show(ctx, |_ui| {

        });
        centered_fixed_window("new_project")
            .show(ctx, |ui| {
                if clickable_label(ui, egui_phosphor::regular::ARROW_LEFT).clicked() {
                    next_state = Some(AppState::SplashScreen(SplashScreen::new()));
                }

                let mut project_path_valid = false;
                key_value_layout(ui, |ui| {
                    project_path_valid = self.project_path_settings(ui);
                    ui.add_space(12.0);
                    ui.end_row();
                    self.project_settings(ui);
                });

                ui.add_space(20.0);
                ui.vertical_centered(|ui| {
                    if ui.add_enabled(project_path_valid, egui::Button::new("Create!")).clicked() {
                        match ProjectClient::local_create_project(self.new_project_path(), self.project_fps, self.project_sample_rate) {
                            Some((client, project)) => next_state = Some(AppState::Editor(Editor::new(project, client, systems))),
                            None => next_state = Some(AppState::Error("Could not create project.".to_owned()))
                        }
                    }
                });
            });

        next_state
    }

    fn project_path_settings(&mut self, ui: &mut egui::Ui) -> bool {
        key_value_row(ui, "Name:", |ui| {
            ui.add(egui::TextEdit::singleline(&mut self.project_name).hint_text("My Awesome Animation"));
        });
        key_value_row(ui, "Location:", |ui| {
            path_selector(ui, &mut self.project_location, true, |_| {});
        });

        if self.project_name.is_empty() {
            key_value_row(ui, "", |ui| {
                ui.add(error_label(ui, "Project name cannot be empty."));
            });
            key_value_row(ui, "", |_| {});
            return false;
        }

        let project_path = self.new_project_path();
        if project_path.exists() {
            key_value_row(ui, "", |ui| {
                ui.add(error_label(ui, "Project already exists at:"));
            });
            key_value_row(ui, "", |ui| {
                ui.allocate_ui(egui::vec2(400.0, ui.available_size_before_wrap().y), |ui| {
                    ui.add(error_label(ui, project_path.to_string_lossy()).truncate(true));
                });
            });
            return false;
        }

        key_value_row(ui, "", |ui| {
            ui.label("Project will be saved to:");
        });
        key_value_row(ui, "", |ui| {
            ui.add(egui::Label::new(format!("{}", project_path.to_string_lossy())).truncate(true));
        });

        true
    }

    fn project_settings(&mut self, ui: &mut egui::Ui) {
        key_value_row(ui, "Frame Rate:", |ui| {
            egui::ComboBox::new(ui.next_auto_id(), "")
                .selected_text(format!("{}", self.project_fps)).show_ui(ui, |ui| {
                    for fps_option in FPS_OPTIONS {
                        ui.selectable_value(&mut self.project_fps, fps_option, format!("{}", fps_option));
                    }
            });
        });
        key_value_row(ui, "Sample Rate:", |ui| {
            egui::ComboBox::new(ui.next_auto_id(), "")
                .selected_text(format!("{}", self.project_sample_rate)).show_ui(ui, |ui| {
                    for sample_rate_option in SAMPLE_RATE_OPTIONS {
                        ui.selectable_value(&mut self.project_sample_rate, sample_rate_option, format!("{}", sample_rate_option));
                    }
            });
        });
    }

    fn new_project_path(&self) -> PathBuf {
        self.project_location.join(self.project_name.clone()).with_extension("cip")
    }

}
