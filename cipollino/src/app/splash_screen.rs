
use std::{path::PathBuf, str::FromStr};

use cipollino_project::client::ProjectClient;
use include_lines::static_include_lines;
use rand::Rng;

#[cfg(not(target_arch = "wasm32"))]
use crate::app::new_project::NewProject;
use crate::editor::Editor;

use super::{collab::Collab, util::centered_fixed_window, AppState, AppSystems};
static_include_lines!(QUOTES, "cipollino/res/quotes.txt");

pub struct SplashScreen {
    quote: &'static str
}

impl Default for SplashScreen {

    fn default() -> Self {
        Self {
            quote: QUOTES[rand::thread_rng().gen_range(0..QUOTES.len())] 
        }
    }

}

impl SplashScreen {

    pub fn new() -> Self {
        Self::default()
    } 

    fn splash_screen_left_side(&mut self, ui: &mut egui::Ui, next_state: &mut Option<AppState>, systems: &mut AppSystems) {
        egui::Frame::default().inner_margin(egui::Margin {
            left: ui.available_width() / 6.0,
            right: ui.available_width() / 4.0,
            ..egui::Margin::ZERO
        }).show(ui, |ui| {
            ui.label("");
            #[cfg(not(target_arch = "wasm32"))]
            {
                if ui.link(format!("{} New Project", egui_phosphor::regular::PLUS)).clicked() { 
                    *next_state = Some(AppState::NewProject(NewProject::new()));
                }
                if ui.link(format!("{} Open Project", egui_phosphor::regular::FOLDER)).clicked() { 
                    if let Some(path) = tinyfiledialogs::open_file_dialog("Open Project", "", Some((&["*.cip"], "Cipollino Project"))) {
                        if let Ok(path) = PathBuf::from_str(&path) {
                            match ProjectClient::local_open_project(path) {
                                Some((client, project)) => *next_state = Some(AppState::Editor(Editor::new(project, client, systems))),
                                None => *next_state = Some(AppState::Error("Could not load project.".to_owned())) 
                            }
                        }
                    }
                }
            }
            if ui.link(format!("{} Collab", egui_phosphor::regular::CLOUD)).clicked() {
                *next_state = Some(AppState::Collab(Collab::new(systems)));
            }
        });
    } 

    fn splash_screen_right_side(&mut self, ui: &mut egui::Ui) {
        ui.label("Recent Projects");
        ui.add_space(12.0);
    }

    pub fn update(&mut self, ctx: &egui::Context, systems: &mut AppSystems) -> Option<AppState> {
        let mut next_state = None;

        egui::CentralPanel::default().show(ctx, |_ui| {

        });

        centered_fixed_window("splash_screen")
            .frame(egui::Frame::window(&ctx.style()).inner_margin(egui::Margin::ZERO))
            .show(ctx, |ui| {
                ui.add(egui::Image::new(egui::include_image!("../../res/banner.png")).rounding(egui::Rounding {
                    se: 0.0,
                    sw: 0.0,
                    ..ui.visuals().window_rounding
                }));
                egui::Frame::default().inner_margin(ui.style().spacing.window_margin).show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.allocate_ui(egui::vec2(240.0, ui.available_size_before_wrap().y), |ui| {
                            ui.small(self.quote);
                        });
                    });
                    ui.add_space(15.0);
                    ui.columns(2, |cols| {
                        let left = &mut cols[0];
                        self.splash_screen_left_side(left, &mut next_state, systems);
                    
                        let right = &mut cols[1];
                        self.splash_screen_right_side(right);
                    });
                    ui.add_space(20.0);
                });
            });

        next_state
    }

}
