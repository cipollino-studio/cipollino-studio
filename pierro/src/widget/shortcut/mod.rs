use crate::{Key, Response, UI};

bitflags::bitflags! {

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct KeyModifiers: u8 {
        const COMMAND = 1 << 0;
        const SHIFT = 1 << 1;
    } 

}

impl KeyModifiers {

    pub fn down_globally(&self, ui: &UI) -> bool {
        ui.key_down(&Key::COMMAND) == self.contains(KeyModifiers::COMMAND) &&
        ui.key_down(&Key::SHIFT) == self.contains(KeyModifiers::SHIFT)
    }

    pub fn down(&self, ui: &UI, response: &Response) -> bool {
        response.key_down(ui, &Key::COMMAND) == self.contains(KeyModifiers::COMMAND) &&
        response.key_down(ui, &Key::SHIFT) == self.contains(KeyModifiers::SHIFT)
    }
    
}

pub struct KeyboardShortcut {
    modifiers: KeyModifiers,
    key_lower: Key,
    key_upper: Key
}

impl KeyboardShortcut {

    pub fn new(modifiers: KeyModifiers, key: Key) -> Self {
        Self {
            modifiers,
            key_lower: key.to_lowercase(),
            key_upper: key.to_uppercase(),
        }
    }

    pub fn used_globally(&self, ui: &UI) -> bool {
        self.modifiers.down_globally(ui) && (ui.key_pressed(&self.key_lower) || ui.key_pressed(&self.key_upper))
    }

    pub fn used(&self, ui: &UI, response: &Response) -> bool {
        self.modifiers.down(ui, response) && (response.key_pressed(ui, &self.key_lower) || response.key_pressed(ui, &self.key_upper))
    }

}