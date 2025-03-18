
use crate::Vec2;
use super::{Key, KeyModifiers};

/// The raw input given to the application by the windowing library
pub(crate) struct RawInput {

    /// The amount of time elapsed since the last redraw
    pub(crate) delta_time: f32,

    /// Mouse position in physical pixels. None if the mouse left the window
    pub(crate) mouse_pos: Option<Vec2>,
    /// Is the left mouse button currently down?
    pub(crate) l_mouse_down: bool,
    /// Is the right mouse button currently down?
    pub(crate) r_mouse_down: bool,
    /// How much has the mouse scrolled
    pub(crate) scroll: Vec2,

    /// What keys were pressed this frame?
    pub(crate) keys_pressed: Vec<Key>,
    /// What keys were released this frame?
    pub(crate) keys_released: Vec<Key>,
    /// What key modifiers are down? 
    pub(crate) key_modifiers: KeyModifiers, 
    /// What text was inputted this frame?
    pub(crate) text: String,

    /// Did this app window lose focus?
    pub(crate) lost_focus: bool,

    /// What is the current IME preedit?
    pub(crate) ime_preedit: String,
    /// What IME text input was commited this frame?
    pub(crate) ime_commit: Option<String>,

    /// The current tablet pen pressure
    pub(crate) pressure: f32
}

impl RawInput {

    pub(crate) fn new() -> Self {
        Self {
            delta_time: 0.0,
            mouse_pos: None,
            l_mouse_down: false,
            r_mouse_down: false,
            scroll: Vec2::ZERO,
            keys_pressed: Vec::new(),
            keys_released: Vec::new(),
            key_modifiers: KeyModifiers::empty(),
            text: String::new(),
            lost_focus: false,
            ime_preedit: String::new(),
            ime_commit: None,
            pressure: 1.0
        }
    }

}
