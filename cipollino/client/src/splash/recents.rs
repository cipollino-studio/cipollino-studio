
use std::path::PathBuf;

use crate::{UserPref, UserPrefs};

pub struct Recents;

impl UserPref for Recents {
    type Type = Vec<PathBuf>;

    fn default() -> Self::Type {
        Vec::new()
    }

    fn name() -> &'static str {
        "recents"
    }

}

pub fn add_recent(prefs: &mut UserPrefs, path: PathBuf) {
    let mut recents = prefs.get::<Recents>();
    if let Some(idx) = recents.iter().position(|recent| recent == &path) {
        recents.remove(idx);
    }
    recents.insert(0, path);
    if recents.len() > 3 {
        recents.pop();
    }
    prefs.set::<Recents>(&recents);
}
