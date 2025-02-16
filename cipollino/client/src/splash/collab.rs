
use crate::{AppState, Editor, Socket};

use super::SplashScreenState;

pub(super) struct CollabScreen {
    url: String,
    socket: Option<Socket>,
    error: String,

    connection_icon_idx: usize,
    connection_icon_timer: f32
}

const CONNECTION_ICONS: &[&'static str] = &[
    pierro::icons::CELL_SIGNAL_NONE,
    pierro::icons::CELL_SIGNAL_LOW,
    pierro::icons::CELL_SIGNAL_MEDIUM,
    pierro::icons::CELL_SIGNAL_HIGH,
    pierro::icons::CELL_SIGNAL_FULL,
];

impl CollabScreen {

    pub fn new() -> Self {
        Self {
            url: String::new(),
            socket: None,
            error: String::new(),
            connection_icon_timer: 0.0,
            connection_icon_idx: 0
        }
    }

    pub fn render(&mut self, ui: &mut pierro::UI, next_state: &mut Option<SplashScreenState>, next_app_state: &mut Option<AppState>) {

        // Back button
        if pierro::icon_button(ui, pierro::icons::ARROW_LEFT).mouse_clicked() {
            *next_state = Some(SplashScreenState::Menu);
        }
        pierro::v_spacing(ui, 5.0);

        // URL
        pierro::horizontal_fit_centered(ui, |ui| {
            pierro::label(ui, "URL: ");
            pierro::text_edit(ui, &mut self.url);
        });
        pierro::label(ui, &self.error);
        pierro::v_spacing(ui, 15.0);
        
        pierro::vertical_centered(ui, |ui| {
            if let Some(socket) = &mut self.socket {
                pierro::horizontal_fit_centered(ui, |ui| {
                    pierro::label(ui, "Connecting");
                    pierro::h_spacing(ui, 7.0);
                    pierro::icon(ui, CONNECTION_ICONS[self.connection_icon_idx]);

                    self.connection_icon_timer -= ui.input().delta_time;
                    if self.connection_icon_timer < 0.0 {
                        self.connection_icon_timer = 0.5;
                        self.connection_icon_idx += 1;
                        self.connection_icon_idx %= CONNECTION_ICONS.len();
                    }
                });
                ui.request_redraw();

                if let Some(welcome_msg) = socket.receive() {
                    let socket = self.socket.take().unwrap();
                    if let Some(editor) = Editor::collab(socket, &welcome_msg) {
                        *next_app_state = Some(AppState::Editor(editor));
                    } else {
                        self.error = "Invalid server protocol.".to_owned();
                    }
                } else if let Some(err) = socket.take_error() {
                    self.error = err;
                    self.socket = None;
                } else if socket.closed() {
                    self.error = "Could not connect to server.".to_owned();
                    self.socket = None;
                }
            } else {
                if pierro::button(ui, "Connect").mouse_clicked() {
                    self.error.clear();
                    match Socket::new(self.url.as_str()) {
                        Ok(new_socket) => {
                            self.socket = Some(new_socket);
                        },
                        Err(msg) => {
                            self.error = msg;
                        },
                    }
                }
            }
        });
    }

}
