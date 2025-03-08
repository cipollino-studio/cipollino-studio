use crate::Vec2;

use super::ButtonInput;


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

    pub(crate) fn new() -> Self {
        Self {
            state: ButtonInput::new(),
            press_pos: None,
            dragging: ButtonInput::new(),
        }
    }

    pub(crate) fn update(&mut self, down: bool, mouse_pos: Option<Vec2>, delta_time: f32) {
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
        self.state.clicked() && !self.drag_stopped()
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