
use crate::{Id, LayoutMemory, TSTransform, UIRef, Vec2, UI};

use super::MouseButton;


#[derive(Clone, Copy)]
pub struct Response {
    pub id: Id,
    pub node_ref: UIRef,

    pub hovered: bool,
    pub l_mouse: MouseButton,
    pub r_mouse: MouseButton,
    pub scroll: Vec2,

    pub dnd_hovered: bool,

    pub keyboard_captured: bool
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