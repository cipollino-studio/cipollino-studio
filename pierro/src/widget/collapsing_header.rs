
use crate::{icons, Size, UINodeParams, UI};

use super::{h_spacing, horizontal_fit, icon, label, v_spacing, vertical_fit};

struct CollapsingHeaderMemory {
    open: bool
}

impl Default for CollapsingHeaderMemory {

    fn default() -> Self {
        Self {
            open: false 
        }
    }

}

pub fn collapsing_header<S: Into<String>, F: FnOnce(&mut UI)>(ui: &mut UI, label_text: S, contents: F) {
    let container = ui.node(UINodeParams::new(Size::fit(), Size::fit()));

    ui.with_parent(container.node_ref, |ui| {
        let open = ui.memory().get::<CollapsingHeaderMemory>(container.id).open;
        let (header_response, _) = horizontal_fit(ui, |ui| {
            let icon_text = if open {
                icons::CARET_DOWN
            } else {
                icons::CARET_RIGHT
            };
            icon(ui, icon_text);
            h_spacing(ui, 3.0);
            label(ui, label_text);
        });
        ui.set_sense_mouse(header_response.node_ref, true);

        if header_response.mouse_clicked() {
            let memory = ui.memory().get::<CollapsingHeaderMemory>(container.id);    
            memory.open = !memory.open;
        }

        if open {
            v_spacing(ui, 5.0);
            horizontal_fit(ui, |ui| {
                h_spacing(ui, 15.0);
                vertical_fit(ui, |ui| {
                    contents(ui);
                });
            });
        }

    });
}
