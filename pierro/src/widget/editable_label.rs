
use crate::UI;

use super::{horizontal_fit, label, text_edit::text_edit, TextEditResponse};

pub fn editable_label(ui: &mut UI, text: &mut String) -> TextEditResponse {
    let mut done_editing = false;

    let (_, response) = horizontal_fit(ui, |ui| {
        let id = ui.get_node_id(ui.curr_parent());

        if ui.memory().has::<()>(id) {
            let resp = text_edit(ui, text);
            if resp.response.mouse_pressed_outside(ui) {
                ui.memory().remove::<()>(id);
            }
            if resp.done_editing {
                ui.memory().remove::<()>(id);
                done_editing = true;
            }
            resp.response
        } else {
            let resp = label(ui, text.as_str());
            ui.set_sense_mouse(resp.node_ref, true);
            if resp.mouse_double_clicked() {
                ui.memory().insert(id, ());
            }
            resp
        }
    });

    TextEditResponse {
        response,
        done_editing
    }
}
