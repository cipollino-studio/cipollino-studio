
use std::collections::HashMap;

use crate::Vec2;

mod button_input;
use bitflags::Flags;
pub use button_input::*;

mod mouse_button;
pub use mouse_button::*;

mod raw_input;
pub(crate) use raw_input::*;

mod distribution;

mod response;
pub use response::*;

mod key;
pub use key::*;

mod keyboard;

pub struct Input {
    pub delta_time: f32,

    pub prev_mouse_pos: Option<Vec2>,
    pub mouse_pos: Option<Vec2>,
    pub l_mouse: MouseButton,
    pub r_mouse: MouseButton,
    pub scroll: Vec2,

    /// The current state of all the keys
    keys: HashMap<Key, ButtonInput>,
    /// The keys pressed on this frame
    pub keys_pressed: Vec<Key>,
    /// The keys released on this frame
    pub keys_released: Vec<Key>,
    /// What key modifiers are currently down?
    pub key_modifiers: KeyModifiers,
    /// What text was inputted this frame?
    pub text: String,
    /// Is the keyboard currently captured by a node in the UI tree?
    pub keyboard_captured: bool,

    pub ime_preedit: String,
    pub ime_commit: Option<String>,

    /// The current tablet pen pressure, going from 0.0 to 1.0
    pub pressure: f32
}

/// The memory storing what inputs are provided to a node
pub(crate) struct Interaction {
    pub(crate) hovered: bool,
    pub(crate) l_mouse: MouseButton,
    pub(crate) r_mouse: MouseButton,
    pub(crate) scroll: Vec2,
    pub(crate) dnd_hovered: bool,
    pub(crate) keyboard_captured: bool
}

impl Default for Interaction {

    fn default() -> Self {
        Self {
            hovered: false,
            l_mouse: MouseButton::new(),
            r_mouse: MouseButton::new(),
            scroll: Vec2::ZERO,
            dnd_hovered: false,
            keyboard_captured: false
        }
    }

}

impl Input {

    /// The change in mouse position between the previous and current frame
    pub fn mouse_delta(&self) -> Vec2 {
        let Some(mouse_pos) = self.mouse_pos else { return Vec2::ZERO };
        let Some(prev_mouse_pos) = self.prev_mouse_pos else { return Vec2::ZERO };
        mouse_pos - prev_mouse_pos
    }

    /// Get the state of a key
    pub fn key_state(&self, key: &Key) -> ButtonInput {
        self.keys.get(key).map(|state| *state).unwrap_or(ButtonInput::new())
    }

    /// Is a key down?
    pub fn key_down(&self, key: &Key) -> bool {
        self.key_state(key).down()
    }

    /// Has a key just been pressed?
    pub fn key_pressed(&self, key: &Key) -> bool {
        self.key_state(key).pressed()
    }

    /// Has a key just been released?
    pub fn key_released(&self, key: &Key) -> bool {
        self.key_state(key).released()
    }

    /// Get a mutable reference to the state of a key
    fn key_state_mut(&mut self, key: &Key) -> &mut ButtonInput {
        if !self.keys.contains_key(&key) {
            self.keys.insert(key.clone(), ButtonInput::new());
        }
        self.keys.get_mut(&key).unwrap()
    }

    pub(crate) fn new() -> Self {
        Self {
            delta_time: 0.0,
            prev_mouse_pos: None,
            mouse_pos: None,
            l_mouse: MouseButton::new(),
            r_mouse: MouseButton::new(),
            scroll: Vec2::ZERO,
            keys: HashMap::new(),
            keys_pressed: Vec::new(),
            keys_released: Vec::new(),
            key_modifiers: KeyModifiers::empty(),
            text: String::new(),
            ime_preedit: String::new(),
            ime_commit: None,
            keyboard_captured: false,
            pressure: 1.0
        }
    }

    /// Update the input given the raw input from the window.
    /// Resets the raw input in preparation for the next frame.
    pub(crate) fn update(&mut self, raw_input: &mut RawInput, scale_factor: f32) {
        self.delta_time = raw_input.delta_time;

        self.prev_mouse_pos = self.mouse_pos;
        self.mouse_pos = raw_input.mouse_pos.map(|pos| pos / scale_factor);

        self.l_mouse.update(raw_input.l_mouse_down, self.mouse_pos, self.delta_time);
        self.r_mouse.update(raw_input.r_mouse_down, self.mouse_pos, self.delta_time);

        // If we start dragging, set the mouse position to the previous mouse position
        // so that the drag starting is registered on the same widget where the mouse began
        if self.l_mouse.drag_started() || self.r_mouse.drag_started() {
            self.mouse_pos = self.prev_mouse_pos;
        }
        
        self.scroll = raw_input.scroll / scale_factor;
        raw_input.scroll = Vec2::ZERO;

        self.keys_pressed = std::mem::replace(&mut raw_input.keys_pressed, Vec::new());
        self.keys_released = std::mem::replace(&mut raw_input.keys_released, Vec::new());
        for (_key, state) in self.keys.iter_mut() {
            state.tick_with_same_state(raw_input.delta_time);
        }
        for key in self.keys_pressed.clone() {
            self.key_state_mut(&key).press();
        }
        for key in self.keys_released.clone() {
            self.key_state_mut(&key).release();
        }
        self.key_modifiers = raw_input.key_modifiers;
        self.text = std::mem::replace(&mut raw_input.text, String::new());

        self.ime_preedit = raw_input.ime_preedit.clone();
        self.ime_commit = std::mem::replace(&mut raw_input.ime_commit, None);

        self.pressure = raw_input.pressure;

        if raw_input.lost_focus {

            self.mouse_pos = None;
            self.l_mouse.update(false, None, 0.0);
            self.r_mouse.update(false, None, 0.0);

            self.scroll = Vec2::ZERO;

            self.keys_pressed.clear();
            self.keys_released.clear();
            for (key, _) in &self.keys {
                self.keys_released.push(key.clone());
            }
            self.keys.clear();
            self.key_modifiers.clear();
            
            raw_input.lost_focus = false;
        }

    }

}
