
use crate::{CursorIcon, Response, UI};

use super::text_edit::{editing_text, text_edit_base, text_edit_begin_editing, text_edit_interaction};

#[derive(Default)]
struct DragValueMemory {
    drag: f32,
}

pub struct DragValueResponse {
    pub drag_value: Response,
    pub done_editing: bool
}

pub fn drag_value(ui: &mut UI, val: &mut i32) -> DragValueResponse {

    let drag_value = text_edit_base(ui, 50.0);
    let mut done_editing = false;

    if drag_value.mouse_clicked() && !drag_value.is_focused(ui) {
        text_edit_begin_editing(ui, &drag_value, &mut val.to_string());
    }

    let drag_delta = drag_value.drag_delta(ui);
    if drag_value.drag_started() {
        drag_value.request_focus(ui);
        let memory = ui.memory().get::<DragValueMemory>(drag_value.id);
        memory.drag = 0.0;
    }
    if !editing_text(ui, &drag_value) {
        if drag_value.drag_stopped() {
            drag_value.release_focus(ui);
            done_editing = true;
        }
    }

    if !editing_text(ui, &drag_value) {
        let memory = ui.memory().get::<DragValueMemory>(drag_value.id);
        memory.drag += drag_delta.x;
        while memory.drag > 5.0 {
            *val += 1;
            memory.drag -= 5.0;
        }
        while memory.drag < -5.0 {
            *val -= 1;
            memory.drag += 5.0;
        }
    }

    if drag_value.hovered {
        let editing_text = editing_text(ui, &drag_value);
        ui.set_cursor(if editing_text {
            CursorIcon::Text
        } else {
            CursorIcon::EwResize
        });
    }

    let mut text = val.to_string();
    let text_edit_response = text_edit_interaction(ui, drag_value, &mut text);
    if text_edit_response.done_editing {
        if let Ok(new_val) = i32::from_str_radix(&text, 10) {
            *val = new_val;
        }
    }
    done_editing |= text_edit_response.done_editing;

    DragValueResponse {
        drag_value,
        done_editing
    }
}
