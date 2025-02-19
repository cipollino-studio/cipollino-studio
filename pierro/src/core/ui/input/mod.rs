
use std::collections::HashMap;

use crate::{TSTransform, Vec2};

use super::{Id, LayoutMemory, Memory, UIRef, UI};

mod key;
pub use key::*;

/// The state of an input device that can either be down or not.
/// Has utilities for detecting the frame the button was first pressed, released, etc.
#[derive(Clone, Copy)]
pub struct ButtonInput {
    state: bool,
    prev_state: bool,
    clicked: bool,
    time_since_press: f32,
    time_since_release: f32,
    click_count: u32
}

impl ButtonInput {

    pub fn new() -> Self {
        Self {
            state: false,
            prev_state: false,
            clicked: false,
            time_since_press: 0.0,
            time_since_release: 0.0,
            click_count: 0
        }
    }

    fn timer_tick(&mut self, delta_time: f32) {
        self.time_since_press += delta_time;
        self.time_since_release += delta_time;
        if self.pressed() {
            self.time_since_press = 0.0;
        }
        if self.released() {
            self.clicked = self.time_since_press < 1.0;
            if self.time_since_release > 0.3 {
                self.click_count = 0;
            }
            if self.clicked {
                self.click_count += 1;
            }
            self.time_since_release = 0.0;
        } else {
            self.clicked = false;
        }
    }

    /// Update the button with a new state
    pub fn tick(&mut self, state: bool, delta_time: f32) {
        self.prev_state = self.state;
        self.state = state;
        self.timer_tick(delta_time);
    } 

    /// Set the button to be down 
    pub fn press(&mut self) {
        self.state = true;
    }

    /// Set the button to be up
    pub fn release(&mut self) {
        self.state = false;
    }

    /// Update the button without providing a new state
    pub fn tick_with_same_state(&mut self, delta_time: f32) {
        self.prev_state = self.state;
        self.timer_tick(delta_time);
    }

    /// Is the button down?
    pub fn down(&self) -> bool {
        self.state
    }

    /// Has the button just been pressed?
    pub fn pressed(&self) -> bool {
        self.state && !self.prev_state
    } 

    /// Has the button just been released?
    pub fn released(&self) -> bool {
        !self.state && self.prev_state
    }

    pub fn click_count(&self) -> u32 {
        if self.clicked {
            self.click_count
        } else {
            0
        }
    }

    /// Was the button clicked?
    pub fn clicked(&self) -> bool {
        self.clicked
    }

    /// Was the button clicked once?
    pub fn single_clicked(&self) -> bool {
        self.click_count() == 1
    }

    /// Was the button double clicked?
    pub fn double_clicked(&self) -> bool {
        self.click_count() == 2
    }

    /// Was the button triple clicked?
    pub fn triple_clicked(&self) -> bool {
        self.click_count() == 3
    }

}

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

    /// What is the current IME preedit?
    pub(crate) ime_preedit: String,
    /// What IME text input was commited this frame?
    pub(crate) ime_commit: Option<String>
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
            ime_preedit: String::new(),
            ime_commit: None
        }
    }

}

/// The state of a mouse button
#[derive(Clone, Copy)]
pub struct MouseButton {
    /// Is the mouse button pressed?
    pub state: ButtonInput,
    /// The position of the mouse when this button was first pressed. `None` if the button is not pressed. 
    pub press_pos: Option<Vec2>,
    /// Is this mouse button being dragged?
    pub dragging: ButtonInput
}

impl MouseButton {

    fn new() -> Self {
        Self {
            state: ButtonInput::new(),
            press_pos: None,
            dragging: ButtonInput::new(),
        }
    }

    fn update(&mut self, down: bool, mouse_pos: Option<Vec2>, delta_time: f32) {
        self.state.tick(down, delta_time);
        self.dragging.tick_with_same_state(delta_time);
        if self.state.pressed() {
            self.press_pos = mouse_pos;
        }
        if self.state.down() {
            if mouse_pos.unwrap_or(Vec2::INFINITY).distance(self.press_pos.unwrap_or(Vec2::INFINITY)) > 2.5 {
                self.dragging.press();
            }
        }
        if self.state.released() {
            self.press_pos = None;
            self.dragging.release();
        }
    }

    pub fn down(&self) -> bool {
        self.state.down()
    }

    pub fn pressed(&self) -> bool {
        self.state.pressed()
    }

    pub fn released(&self) -> bool {
        self.state.released()
    }

    pub fn clicked(&self) -> bool {
        self.state.clicked()
    }

    pub fn double_clicked(&self) -> bool {
        self.state.double_clicked()
    }

    pub fn triple_clicked(&self) -> bool {
        self.state.triple_clicked()
    }

    pub fn dragging(&self) -> bool {
        self.dragging.down()
    }

    pub fn drag_started(&self) -> bool {
        self.dragging.pressed()
    }

    pub fn drag_stopped(&self) -> bool {
        self.dragging.released()
    }

}

pub struct Input {
    pub delta_time: f32,

    pub prev_mouse_pos: Option<Vec2>,
    pub mouse_pos: Option<Vec2>,
    pub l_mouse: MouseButton,
    pub r_mouse: MouseButton,
    pub scroll: Vec2,

    keys: HashMap<Key, ButtonInput>,
    pub keys_pressed: Vec<Key>,
    pub keys_released: Vec<Key>,

    pub ime_preedit: String,
    pub ime_commit: Option<String>
}

/// The memory storing what inputs are provided to a node
pub(crate) struct Interaction {
    pub(crate) hovered: bool,
    pub(crate) through_hovered: bool,
    pub(crate) l_mouse: MouseButton,
    pub(crate) r_mouse: MouseButton,
    pub(crate) scroll: Vec2
}

impl Default for Interaction {

    fn default() -> Self {
        Self {
            hovered: false,
            through_hovered: false,
            l_mouse: MouseButton::new(),
            r_mouse: MouseButton::new(),
            scroll: Vec2::ZERO
        }
    }

}

fn find_interacted_node<F: Fn(&mut LayoutMemory) -> bool>(memory: &mut Memory, node: Id, pos: Vec2, ignore: Option<Id>, criteria: &F) -> Option<Id> {
    // Check priority nodes first
    let mut child = memory.get::<LayoutMemory>(node).first_child;
    while let Some(child_id) = child {
        if memory.get::<LayoutMemory>(child_id).has_interaction_priority {
            if let Some(node) = find_interacted_node(memory, child_id, pos, ignore, criteria) {
                return Some(node);
            }
        }
        child = memory.get::<LayoutMemory>(child_id).next;
    } 

    // Then check non-priority nodes
    let mut child = memory.get::<LayoutMemory>(node).first_child;
    while let Some(child_id) = child {
        if !memory.get::<LayoutMemory>(child_id).has_interaction_priority {
            if let Some(node) = find_interacted_node(memory, child_id, pos, ignore, criteria) {
                return Some(node);
            }
        }
        child = memory.get::<LayoutMemory>(child_id).next;
    }

    let layout_mem = memory.get::<LayoutMemory>(node);
    if layout_mem.interaction_rect.contains(pos) && criteria(layout_mem) && Some(node) != ignore {
        return Some(node);
    }

    None

}

/// Find the node hovered at a given position in screen space
fn find_hover_node(memory: &mut Memory, node: Id, pos: Vec2, ignore: Option<Id>) -> Option<Id> {
    find_interacted_node(memory, node, pos, ignore, &|mem| mem.sense_mouse)
}

/// Find the scrollable at a given position in screen space
fn find_scrollable_node(memory: &mut Memory, node: Id, pos: Vec2, ignore: Option<Id>) -> Option<Id> {
    find_interacted_node(memory, node, pos, ignore, &|mem| mem.sense_scroll)
}

impl Input {

    /// The change in mouse position between the previous and current frame
    pub fn mouse_delta(&self) -> Vec2 {
        let Some(mouse_pos) = self.mouse_pos else { return Vec2::ZERO };
        let Some(prev_mouse_pos) = self.prev_mouse_pos else { return Vec2::ZERO };
        mouse_pos - prev_mouse_pos
    }

    /// Get the state of a key
    pub fn key_state(&self, key: Key) -> ButtonInput {
        self.keys.get(&key).map(|state| *state).unwrap_or(ButtonInput::new())
    }

    /// Is a key down?
    pub fn key_down(&self, key: Key) -> bool {
        self.key_state(key).down()
    }

    /// Has a key just been pressed?
    pub fn key_pressed(&self, key: Key) -> bool {
        self.key_state(key).pressed()
    }

    /// Has a key just been released?
    pub fn key_released(&self, key: Key) -> bool {
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
            ime_preedit: String::new(),
            ime_commit: None
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
        for key in self.keys_pressed.clone() {
            self.key_state_mut(&key).press();
        }
        for key in self.keys_released.clone() {
            self.key_state_mut(&key).release();
        }
        for (_key, state) in self.keys.iter_mut() {
            state.tick_with_same_state(raw_input.delta_time);
        }

        self.ime_preedit = raw_input.ime_preedit.clone();
        self.ime_commit = std::mem::replace(&mut raw_input.ime_commit, None);

    }

    /// Distribute the input to nodes, taking foucs into account.
    pub(crate) fn distribute(&self, memory: &mut Memory) {

        // Take away focus if we clicked outside the focused node
        if let Some(focused_node) = memory.get_focus() {
            let focused_node_memory = memory.get::<LayoutMemory>(focused_node);
            if let Some(mouse_pos) = self.mouse_pos {
                if (self.l_mouse.pressed() || self.r_mouse.pressed()) && !focused_node_memory.interaction_rect.contains(mouse_pos) {
                    memory.release_focus();
                }
            }
        }

        let layer_ids = memory.layer_ids.clone();
        let hovered_node = memory.get_focus().or_else(|| {
            let mouse_pos = self.mouse_pos?;
            for layer in layer_ids.iter().rev() { 
                if let Some(hovered_node) = find_hover_node(memory, *layer, mouse_pos, None) {
                    return Some(hovered_node)
                }
            }
            None
        });
        let scrollable_node = (|| {
            let mouse_pos = self.mouse_pos?;
            for layer in layer_ids.iter().rev() { 
                if let Some(scrollable_node) = find_scrollable_node(memory, *layer, mouse_pos, None) {
                    return Some(scrollable_node)
                }
            }
            None
        })();
        let through_hovered_node = self.mouse_pos.map(|mouse_pos| {
            for layer in layer_ids.iter().rev() { 
                if let Some(hovered_node) = find_hover_node(memory, *layer, mouse_pos, memory.get_focus()) {
                    return Some(hovered_node)
                }
            }
            None
        }).flatten();

        for (id, interaction) in memory.iter_mut::<Interaction>() {
            let hovered = Some(id) == hovered_node;
            let scrollable = Some(id) == scrollable_node;
            interaction.hovered = hovered;
            interaction.through_hovered = Some(id) == through_hovered_node;
            interaction.l_mouse = if hovered { self.l_mouse } else { MouseButton::new() };
            interaction.r_mouse = if hovered { self.r_mouse } else { MouseButton::new() };
            interaction.scroll = if scrollable { self.scroll } else { Vec2::ZERO };
        }

        // If we're not holding the mouse down, we can't be drag and dropping anything
        if !self.l_mouse.down() && !self.l_mouse.released() {
            memory.clear_dnd_payload();
        }
    }

}

#[derive(Clone, Copy)]
pub struct Response {
    pub id: Id,
    pub node_ref: UIRef,

    pub hovered: bool,
    /// Is this node being hovered through the currently focused node?
    pub through_hovered: bool,
    pub l_mouse: MouseButton,
    pub r_mouse: MouseButton,
    pub scroll: Vec2
}

impl Response {

    pub fn contains_mouse(&self, ui: &mut UI) -> bool {
        let Some(pos) = ui.input().mouse_pos else { return false; };
        ui.memory().get::<LayoutMemory>(self.id).screen_rect.contains(pos)
    }

    /// Returns the position of the mouse relative to the node
    pub fn mouse_pos(&self, ui: &mut UI) -> Option<Vec2> {
        let screen_pos = ui.input().mouse_pos?;
        let layout_memory = ui.memory().get::<LayoutMemory>(self.id);
        let rect = layout_memory.screen_rect;
        let scale = layout_memory.transform.scale;
        Some((screen_pos - rect.tl()) / scale) 
    }

    pub fn mouse_down(&self) -> bool {
        self.l_mouse.down()
    }

    pub fn mouse_pressed(&self) -> bool {
        self.l_mouse.pressed()
    }

    pub fn mouse_released(&self) -> bool {
        self.l_mouse.released()
    }

    pub fn mouse_clicked(&self) -> bool {
        self.l_mouse.clicked()
    }

    pub fn mouse_double_clicked(&self) -> bool {
        self.l_mouse.double_clicked()
    }

    pub fn mouse_triple_clicked(&self) -> bool {
        self.l_mouse.triple_clicked()
    }

    pub fn dragging(&self) -> bool {
        self.l_mouse.dragging()
    }

    pub fn drag_started(&self) -> bool {
        self.l_mouse.drag_started()
    }

    pub fn drag_stopped(&self) -> bool {
        self.l_mouse.drag_stopped()
    }

    pub fn drag_delta(&self, ui: &mut UI) -> Vec2 {
        if !self.dragging() {
            return Vec2::ZERO;
        }
        let scale = self.scale(ui);
        ui.input().mouse_delta() / scale
    }

    pub fn right_mouse_down(&self) -> bool {
        self.r_mouse.down()
    }

    pub fn right_mouse_pressed(&self) -> bool {
        self.r_mouse.pressed()
    }

    pub fn right_mouse_released(&self) -> bool {
        self.r_mouse.released()
    }

    pub fn right_mouse_clicked(&self) -> bool {
        self.r_mouse.clicked()
    }

    pub fn right_mouse_double_clicked(&self) -> bool {
        self.r_mouse.double_clicked()
    }

    pub fn right_mouse_triple_clicked(&self) -> bool {
        self.r_mouse.triple_clicked()
    }

    pub fn right_dragging(&self) -> bool {
        self.r_mouse.dragging()
    }

    pub fn right_drag_started(&self) -> bool {
        self.r_mouse.drag_started()
    }

    pub fn right_drag_stopped(&self) -> bool {
        self.r_mouse.drag_stopped()
    }

    pub fn right_drag_delta(&self, ui: &mut UI) -> Vec2 {
        if !self.right_dragging() {
            return Vec2::ZERO;
        }
        let scale = self.scale(ui);
        ui.input().mouse_delta() / scale
    }

    pub fn mouse_pressed_outside(&self, ui: &mut UI) -> bool {
        (ui.input().l_mouse.pressed() || ui.input().r_mouse.pressed()) && !self.contains_mouse(ui)
    }

    pub fn is_focused(&self, ui: &mut UI) -> bool {
        ui.memory().is_focused(self.id)
    }

    pub fn request_focus(&self, ui: &mut UI) {
        ui.memory().request_focus(self.id);
    }

    pub fn release_focus(&self, ui: &mut UI) {
        if self.is_focused(ui) {
            ui.memory().release_focus();
        }
    }

    pub fn transform(&self, ui: &mut UI) -> TSTransform {
        ui.memory().get::<LayoutMemory>(self.id).transform
    }

    pub fn scale(&self, ui: &mut UI) -> f32 {
        self.transform(ui).scale
    }

}
