
use crate::{Key, KeyModifiers, Response, UI};

mod label;
pub use label::*;

mod rebind;
pub use rebind::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct KeyboardShortcut {
    modifiers: KeyModifiers,
    key: Key
}

impl KeyboardShortcut {

    pub const fn new(modifiers: KeyModifiers, key: Key) -> Self {
        Self {
            modifiers,
            key
        }
    }

    pub fn used_globally(&self, ui: &UI) -> bool {
        ui.input().key_modifiers == self.modifiers && ui.key_pressed(&self.key)
    }

    pub fn used(&self, ui: &UI, response: &Response) -> bool {
        ui.input().key_modifiers == self.modifiers && response.key_pressed(ui, &self.key)
    }

}