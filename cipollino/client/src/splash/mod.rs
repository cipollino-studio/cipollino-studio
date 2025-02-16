
mod collab;
use collab::CollabScreen;

mod menu;
use menu::menu;

use crate::AppState;

enum SplashScreenState {
    Menu,
    Collab(CollabScreen)
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

    pub fn tick(&mut self, ui: &mut pierro::UI, next_app_state: &mut Option<AppState>) {
        let mut next_state = None;

        pierro::modal(ui, |ui| {
            match &mut self.state {
                SplashScreenState::Menu => {
                    menu(ui, &mut next_state, next_app_state); 
                },
                SplashScreenState::Collab(collab) => {
                    collab.render(ui, &mut next_state, next_app_state);
                },
            }
        });

        if let Some(next_state) = next_state {
            self.state = next_state;
        }
    }

}
