
use std::io::Write;
use std::{io::Read, path::PathBuf};
use std::fs;

use super::{UserPref, UserPrefs};

pub fn read_json_file(path: &PathBuf) -> Option<serde_json::Value> {
    let mut file = fs::File::open(path).ok()?;
    let mut str = "".to_owned();
    file.read_to_string(&mut str).ok()?;
    serde_json::from_str::<serde_json::Value>(str.as_str()).ok()
}

pub fn write_json_file(path: &PathBuf, data: serde_json::Value) -> Option<()> {
    let str = data.to_string();
    let mut file = fs::File::create(path).ok()?;
    file.write(str.as_bytes()).ok()?;
    Some(())
}

pub fn get_prefs_path() -> PathBuf {
    let config_path = directories::ProjectDirs::from("com", "Cipollino", "Cipollino").unwrap().config_dir().to_owned();
    config_path.join("prefs.json")
}

impl UserPrefs {

    pub fn new() -> Self {
        let prefs_path = get_prefs_path();
        let prefs = if let Some(json) = read_json_file(&prefs_path) {
            if let Some(map) = json.as_object() {
                map.clone()
            } else {
                serde_json::Map::new()
            }
        } else {
            write_json_file(&prefs_path, serde_json::json!({}));
            serde_json::Map::new()
        };

        Self {
            prefs_path,
            prefs
        }
    }

    fn get_existing<P>(&self) -> Option<P::Type> where P: UserPref {
        let val = self.prefs.get(P::name())?;
        let val_parsed = serde_json::from_value::<P::Type>(val.clone()).ok()?;
        Some(val_parsed)
    }

    pub fn get<P>(&mut self) -> P::Type where P: UserPref {
        if let Some(val) = self.get_existing::<P>() {
            val
        } else {
            self.set::<P>(&P::default());
            P::default()
        }
    }

    pub fn set<P>(&mut self, val: &P::Type) where P: UserPref {
        self.prefs.insert(P::name().to_owned(), serde_json::json!(val));
        write_json_file(&self.prefs_path, serde_json::json!(self.prefs));
    }

}
