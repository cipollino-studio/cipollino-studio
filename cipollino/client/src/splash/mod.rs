
mod collab;
use collab::CollabScreen;

mod menu;
use menu::menu;

mod new_project;
use new_project::NewProjectScreen;

mod recents;

use crate::{AppState, AppSystems};

enum SplashScreenState {
    Menu,
    NewProject(NewProjectScreen),
    Collab(CollabScreen),
}

pub struct SplashScreen {
    state: SplashScreenState
}

impl SplashScreen {

    pub fn new() -> Self {
        Self {
            state: SplashScreenState::Menu
        }
    }

    pub fn tick(&mut self, ui: &mut pierro::UI, next_app_state: &mut Option<AppState>, systems: &mut AppSystems) {
        let mut next_state = None;

        pierro::modal(ui, |ui| {
            match &mut self.state {
                SplashScreenState::Menu => {
                    menu(ui, &mut next_state, next_app_state, systems); 
                },
                SplashScreenState::NewProject(new_project) => {
                    new_project.render(ui, &mut next_state, next_app_state, systems);
                },
                SplashScreenState::Collab(collab) => {
                    collab.render(ui, &mut next_state, next_app_state, systems);
                },
            }
        });

        if let Some(next_state) = next_state {
            self.state = next_state;
        }
    }

}
