
mod prefs;
pub use prefs::*;

mod shortcut;
pub use shortcut::*;

use std::path::PathBuf;

use crate::AudioEngine;

pub struct AppSystems {
    pub home_path: PathBuf,
    pub app_data_path: PathBuf,
    pub audio_tmp_path: PathBuf,
    pub prefs: UserPrefs,
    pub audio: AudioEngine
}

impl AppSystems {

    pub fn new() -> Self {

        let user_dirs = directories::UserDirs::new()
            .expect("could not get user dirs");

        let home_path = user_dirs.home_dir().to_owned();

        let app_data_path = directories::ProjectDirs::from("org", "cipollino", "Cipollino")
            .expect("could not get app data path")
            .config_dir()
            .to_owned(); 
        let _ = std::fs::create_dir_all(&app_data_path);

        let audio_tmp_path = app_data_path.join("audio_tmp");
        let _ = std::fs::create_dir_all(&audio_tmp_path);
        // Delete anything in the audio tmp directory just in case
        if let Ok(entries) = std::fs::read_dir(&audio_tmp_path) {
            for entry in entries.into_iter() {
                let Ok(entry) = entry else { continue; }; 
                let _ = std::fs::remove_file(entry.path());
            }
        }

        let prefs = UserPrefs::new(app_data_path.join("prefs.json"));

        let audio = AudioEngine::new().expect("could not initialize audio engine");

        Self {
            home_path,
            app_data_path,
            audio_tmp_path,
            prefs,
            audio
        }
    }

}

