
use crate::{CursorIcon, Numeric, Response, UI};

use super::text_edit::{editing_text, text_edit_base, text_edit_begin_editing, text_edit_interaction};

#[derive(Default)]
struct DragValueMemory {
    drag: f32,
}

pub struct DragValueResponse {
    pub drag_value: Response,
    pub done_editing: bool,
    pub editing: bool
}

pub struct DragValue<'value, N: Numeric> {
    val: &'value mut N,
    step: N,
    pixels_per_step: f32,
    min: N,
    max: N
}

impl<'value, N: Numeric> DragValue<'value, N> {

    pub fn new(val: &'value mut N) -> Self {
        Self {
            val,
            step: if N::INTEGRAL {
                N::from_f64(1.0) 
            } else {
                N::from_f64(0.05) 
            },
            pixels_per_step: if N::INTEGRAL {
                5.0
            } else {
                1.0
            },
            min: N::MIN,
            max: N::MAX
        } 
    }

    pub fn with_min(mut self, min: N) -> Self {
        self.min = min;
        self
    }

    pub fn with_max(mut self, max: N) -> Self {
        self.max = max;
        self
    }

    pub fn render(self, ui: &mut UI) -> DragValueResponse {
        
        let drag_value = text_edit_base(ui, 50.0);
        let mut done_editing = false;

        if drag_value.mouse_clicked() && !drag_value.is_focused(ui) {
            text_edit_begin_editing(ui, drag_value.id, &mut self.val.to_str());
        }

        let drag_delta = drag_value.drag_delta(ui);
        if drag_value.drag_started() {
            drag_value.request_focus(ui);
            let memory = ui.memory().get::<DragValueMemory>(drag_value.id);
            memory.drag = 0.0;
        }
        if !editing_text(ui, drag_value.id) {
            if drag_value.drag_stopped() {
                drag_value.release_focus(ui);
                done_editing = true;
            }
        }

        if !editing_text(ui, drag_value.id) {
            let memory = ui.memory().get::<DragValueMemory>(drag_value.id);
            memory.drag += drag_delta.x;
            while memory.drag > self.pixels_per_step {
                if *self.val < self.max - self.step {
                    *self.val = *self.val + self.step;
                } else {
                    *self.val = self.max;
                }
                memory.drag -= self.pixels_per_step;
            }
            while memory.drag < -self.pixels_per_step {
                if *self.val > self.min + self.step {
                    *self.val = *self.val - self.step;
                } else {
                    *self.val = self.min;
                }
                memory.drag += self.pixels_per_step;
            }
        }

        if drag_value.hovered {
            let editing_text = editing_text(ui, drag_value.id);
            ui.set_cursor(if editing_text {
                CursorIcon::Text
            } else {
                CursorIcon::EwResize
            });
        }

        let mut text = self.val.to_str();
        let text_edit_response = text_edit_interaction(ui, drag_value, &mut text);
        if text_edit_response.done_editing {
            if let Ok(mut new_val) = N::from_str(&text) {
                if new_val > self.max {
                    new_val = self.max;
                }
                if new_val < self.min {
                    new_val = self.min;
                }
                *self.val = new_val;
            }
        }
        done_editing |= text_edit_response.done_editing;

        DragValueResponse {
            drag_value,
            done_editing,
            editing: editing_text(ui, drag_value.id) || drag_value.dragging()
        }

    }

}

pub fn drag_value<N: Numeric>(ui: &mut UI, val: &mut N) -> DragValueResponse {
    DragValue::new(val).render(ui)
}
