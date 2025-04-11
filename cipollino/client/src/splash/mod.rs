
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

include_lines::static_include_lines!(QUOTES, "cipollino/client/res/quotes.txt");

pub struct SplashScreen {
    state: SplashScreenState,
    quote: &'static str,
    error: Option<String>
}

impl SplashScreen {

    pub fn new() -> Self {
        use rand::Rng;
        Self {
            state: SplashScreenState::Menu,
            quote: QUOTES[rand::rng().random_range(0..QUOTES.len())],
            error: None 
        }
    }

    pub fn new_with_error(msg: String) -> Self {
        use rand::Rng;
        Self {
            state: SplashScreenState::Menu,
            quote: QUOTES[rand::rng().random_range(0..QUOTES.len())],
            error: Some(msg)
        }
    }

    pub fn tick(&mut self, ui: &mut pierro::UI, next_app_state: &mut Option<AppState>, systems: &mut AppSystems) {
        let mut next_state = None;

        // Background
        let bg_color = ui.style::<pierro::theme::BgLight>();
        ui.node(
            pierro::UINodeParams::new(pierro::Size::fr(1.0), pierro::Size::fr(1.0))
                .with_fill(bg_color)
        );

        let rounding = 7.5;
        let (response, banner_width) = pierro::modal(ui, |ui| {
            
            // Banner
            let texture = pierro::include_image!(ui, "../../res/banner.png");
            let banner_scale = 0.4;
            let banner_width = banner_scale * texture.width() as f32;
            let banner_image = pierro::scaled_image(ui, banner_scale, texture);
            ui.set_rounding(banner_image.node_ref, pierro::Rounding::top(7.5 - 1.0));
            pierro::h_line(ui);

            // Quote
            pierro::v_spacing(ui, 5.0);
            pierro::vertical_centered(ui, |ui| {
                ui.with_style::<pierro::theme::LabelFontSize, _, _>(12.0, |ui| {
                    pierro::label(ui, self.quote);
                });
            });

            pierro::margin_with_size(ui, pierro::Margin::same(10.0), pierro::Size::fr(1.0), pierro::Size::fit(), |ui| {
                match &mut self.state {
                    SplashScreenState::Menu => {
                        menu(ui, &mut next_state, next_app_state, &mut self.error, systems); 
                    },
                    SplashScreenState::NewProject(new_project) => {
                        new_project.render(ui, &mut next_state, next_app_state, &mut self.error, systems);
                    },
                    SplashScreenState::Collab(collab) => {
                        collab.render(ui, &mut next_state, next_app_state, systems);
                    },
                }
            });

            banner_width
        });
        ui.set_size(response.node_ref, pierro::Size::px(banner_width), pierro::Size::fit());
        ui.set_rounding(response.node_ref, pierro::Rounding::same(rounding));

        if let Some(error) = &self.error {
            let (_, close) = pierro::modal(ui, |ui| {
                pierro::margin(ui, pierro::Margin::same(5.0), |ui| {
                    // Close button
                    let close = pierro::clickable_icon(ui, pierro::icons::X).mouse_clicked();
                    pierro::v_spacing(ui, 5.0);

                    // Message
                    pierro::margin(ui, pierro::Margin::same(5.0), |ui| {
                        pierro::label(ui, error);
                    });

                    close
                })
            });

            if close {
                self.error = None;
            }
        }
        
        if let Some(next_state) = next_state {
            self.state = next_state;
        }
    }

}
