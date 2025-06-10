
mod splash;

mod editor;
pub use editor::*;

mod panels;
pub use panels::*;

mod tool;
use pierro::include_icon;
pub use tool::*;

mod systems;
pub use systems::*;

mod render;
pub use render::*;

mod util;
pub use util::*;

mod audio;
pub use audio::*;

use std::path::PathBuf;
use clap::Parser;
use splash::SplashScreen;

pub enum AppState {
    SplashScreen(SplashScreen),
    Editor(Editor)
}

struct App {
    state: AppState,
    systems: AppSystems
}

impl pierro::App for App {

    fn window_config() -> pierro::WindowConfig {
        pierro::WindowConfig::default()
            .maximize_window()
            .with_icon(include_icon!("../res/icon.png"))
            .with_title("Cipollino Studio")
    }
    
    fn tick(&mut self, ui: &mut pierro::UI) {
        let mut next_app_state = None;

        match &mut self.state {
            AppState::SplashScreen(splash_screen) => splash_screen.tick(ui, &mut next_app_state, &mut self.systems),
            AppState::Editor(editor) => editor.tick(ui, &mut self.systems, &mut next_app_state),
        }

        if let Some(next_app_state) = next_app_state {
            self.state = next_app_state;
        }
    }

}

#[derive(Parser)]
struct Args {
    #[arg(long)]
    project: Option<PathBuf>,
    #[arg(long)]
    url: Option<String>
}

fn main() {

    rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();

    let mut systems = AppSystems::new();

    let args = Args::parse();

    let app = if let Some(path) = args.project {
        let editor = Editor::local(path, &mut systems).expect("could not open project.");
        App {
            state: AppState::Editor(editor),
            systems
        }
    } else if let Some(url) = args.url {

        let mut socket = Socket::new(url.as_str()).unwrap(); 

        let mut welcome_msg = None;
        while welcome_msg.is_none() {
            if socket.closed() {
                panic!("could not connect to server at {}.", url);
            }
            welcome_msg = socket.receive();
        }
        let welcome_msg = welcome_msg.unwrap();

        let editor = Editor::collab(socket, &welcome_msg, &mut systems).expect("invalid server protocol");

        App {
            state: AppState::Editor(editor),
            systems
        }
    } else {
        App {
            state: AppState::SplashScreen(SplashScreen::new()),
            systems
        }
    };

    // Make sure that the working directory is the folder the executable's in
    // This is important for starting the FFMPEG process when the app is bundled
    #[cfg(not(debug_assertions))]
    use std::env::{current_exe, set_current_dir};
    #[cfg(not(debug_assertions))]
    set_current_dir(current_exe().unwrap().parent().unwrap()).unwrap();

    pierro::run(app);
}
