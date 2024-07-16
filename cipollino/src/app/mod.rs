
mod util;

pub mod splash_screen;
use crate::editor::Editor;
use crate::util::ui::clickable_label;

use self::prefs::UserPrefs;
use self::splash_screen::SplashScreen;

mod collab;
use self::collab::Collab;

#[cfg(not(target_arch = "wasm32"))]
mod new_project;
#[cfg(not(target_arch = "wasm32"))]
use self::new_project::NewProject;

use self::util::centered_fixed_window;

pub mod prefs;

pub enum AppState {
    SplashScreen(SplashScreen),
    #[cfg(not(target_arch = "wasm32"))]
    NewProject(NewProject),
    Error(String),
    Collab(Collab),
    Editor(Editor)
}

pub struct AppSystems<'a> {
    pub prefs: &'a mut UserPrefs
}
pub struct App {
    state: AppState,
    prefs: UserPrefs
}

impl App {

    pub fn new(cc: &eframe::CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);

        App {
            state: AppState::SplashScreen(SplashScreen::new()),
            prefs: UserPrefs::new()
        }
    }

}

impl eframe::App for App {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let mut systems = AppSystems {
            prefs: &mut self.prefs
        };

        ctx.style_mut(|style| {
            style.visuals.window_shadow.offset = egui::Vec2::ZERO;
            style.visuals.popup_shadow.offset = egui::Vec2::ZERO;
        });

        let next_state = match &mut self.state {
            AppState::SplashScreen(splash_screen) => splash_screen.update(ctx, &mut systems),
            #[cfg(not(target_arch = "wasm32"))]
            AppState::NewProject(new_project) => new_project.update(ctx, &mut systems),
            AppState::Collab(collab) => collab.update(ctx, &mut systems),
            AppState::Error(error) => {
                let mut next_state = None;

                egui::CentralPanel::default().show(ctx, |_ui| {

                });
                centered_fixed_window("error")
                    .show(ctx, |ui| {
                        if clickable_label(ui, egui_phosphor::regular::X).clicked() {
                            next_state = Some(AppState::SplashScreen(SplashScreen::new()));
                        }
                        ui.vertical_centered(|ui| {
                            ui.label(error.as_str());
                        });
                    });

                next_state
            }, 
            AppState::Editor(editor) => editor.update(ctx, &mut systems) 
        };
        if let Some(state) = next_state {
            self.state = state;
        }

    }

}
