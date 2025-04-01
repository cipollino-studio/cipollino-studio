
mod prefs;
pub use prefs::*;

mod shortcut;
pub use shortcut::*;

use std::path::PathBuf;

pub struct AppSystems {
    pub home_path: PathBuf,
    pub app_data_path: PathBuf,
    pub prefs: UserPrefs
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

        let prefs = UserPrefs::new(app_data_path.join("prefs.json"));

        Self {
            home_path,
            app_data_path,
            prefs
        }
    }

}

