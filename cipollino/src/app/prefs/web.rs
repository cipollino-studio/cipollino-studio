
use super::{UserPrefs, UserPref};

impl UserPrefs {

    pub fn new() -> Self {
        Self {
            storage: web_sys::window().unwrap().local_storage().unwrap().unwrap()
        }
    }

    fn get_existing<P>(&self) -> Option<P::Type> where P: UserPref {
        let val = self.storage.get_item(P::name()).ok()??;
        let val_parsed = serde_json::from_str::<P::Type>(&val).ok()?;
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
        if let Ok(val_str) = serde_json::to_string(&val) {
            self.storage.set_item(P::name(), &val_str);
        }
    }

}
