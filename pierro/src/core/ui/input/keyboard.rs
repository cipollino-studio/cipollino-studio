
use crate::UI;

use super::{Key, Response};

impl UI<'_, '_> {

    pub fn key_pressed(&self, key: &Key) -> bool {
        !self.input().keyboard_captured && self.input().key_pressed(key)
    }

    pub fn key_down(&self, key: &Key) -> bool {
        !self.input().keyboard_captured && self.input().key_down(key)
    }

    pub fn key_released(&self, key: &Key) -> bool {
        !self.input().keyboard_captured && self.input().key_released(key)
    }

}

impl Response {

    pub fn key_pressed(&self, ui: &UI, key: &Key) -> bool {
        self.keyboard_captured && ui.input().key_pressed(key)
    }

    pub fn key_down(&self, ui: &UI, key: &Key) -> bool {
        self.keyboard_captured && ui.input().key_down(key)
    }

    pub fn key_released(&self, ui: &UI, key: &Key) -> bool {
        self.keyboard_captured && ui.input().key_released(key)
    }

}
