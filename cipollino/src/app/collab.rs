
use cipollino_project::{client::ProjectClient, protocol::Message, socket::{Socket, WsEvent, WsMessage}};

use crate::util::ui::{clickable_label, key_value_layout, key_value_row};

use super::{editor::Editor, prefs::UserPref, splash_screen::SplashScreen, util::centered_fixed_window, AppState, AppSystems};

struct CollabURLPref;

impl UserPref for CollabURLPref {
    type Type = String;

    fn default() -> Self::Type {
        "".to_owned()
    }

    fn name() -> &'static str {
        "collab_url"
    }
}

pub struct Collab {
    url: String,
    pending_socket: Option<Socket>,
    error: String
}

impl Collab {

    pub fn new(systems: &mut AppSystems) -> Self {
        Self {
            url: systems.prefs.get::<CollabURLPref>(),
            pending_socket: None,
            error: "".to_owned()
        }
    }

    fn handle_first_message(&mut self, msg: WsMessage, systems: &mut AppSystems) -> Option<AppState> {

        let error_msg = "Invalid server protocol. Make sure the collab URL is correct.".to_owned();

        match msg {
            cipollino_project::socket::WsMessage::Binary(msg) => {
                let msg = match bson::from_slice::<Message>(&msg).ok() {
                    Some(msg) => msg,
                    None => return Some(AppState::Error(error_msg))
                };
                match msg {
                    Message::Welcome(data) => {
                        let socket = self.pending_socket.take()?; 
                        let (client, project) = ProjectClient::collab(socket, data);
                        Some(AppState::Editor(Editor::new(project, client, systems)))
                    },
                    _ => return Some(AppState::Error(error_msg)) 
                }
            },
            _ => None
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, systems: &mut AppSystems) -> Option<AppState> {
        let mut next_state = None;

        if let Some(socket) = &self.pending_socket {
            while let Some(event) = socket.receive() {
                match event {
                    WsEvent::Message(msg) => {
                        next_state = self.handle_first_message(msg, systems);
                        break;
                    },
                    WsEvent::Error(msg) => {
                        self.error = msg;
                        self.pending_socket = None;
                        break;
                    }, 
                    _ => {} 
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |_ui| {
            
        });
        centered_fixed_window("collab")
            .show(ctx, |ui| {
                if clickable_label(ui, egui_phosphor::regular::ARROW_LEFT).clicked() {
                    next_state = Some(AppState::SplashScreen(SplashScreen::new()));
                }

                key_value_layout(ui, |ui| {
                    key_value_row(ui, "URL:", |ui| {
                        if ui.text_edit_singleline(&mut self.url).changed() {
                            systems.prefs.set::<CollabURLPref>(&self.url);
                        }
                    });
                    key_value_row(ui, "", |ui| {
                        ui.label(egui::RichText::new(self.error.clone()).color(ui.style().visuals.error_fg_color));
                    })
                });
                ui.vertical_centered(|ui| {
                    if self.pending_socket.is_none() { 
                        if ui.button("Connect").clicked() {
                            let ctx_clone = ui.ctx().clone();
                            match Socket::new(&self.url, move || ctx_clone.request_repaint()) {
                                Ok(socket) => self.pending_socket = Some(socket),
                                Err(msg) => self.error = msg,
                            }
                        }
                    } else {
                        ui.spinner();
                    }
                });
            });

        next_state
    }

}
