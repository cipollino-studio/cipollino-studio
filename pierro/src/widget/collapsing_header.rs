
use crate::{icons, Response, Size, UINodeParams, UI};

use super::{button_text_color_animation, h_spacing, horizontal_fit, horizontal_fit_centered, icon, label, theme, v_spacing, vertical_fit};

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

pub fn collapsing_header<H: FnOnce(&mut UI, &Response), F: FnOnce(&mut UI)>(ui: &mut UI, header: H, contents: F) -> Response {
    let container = ui.node(UINodeParams::new(Size::fit(), Size::fit()));

    ui.with_parent(container.node_ref, |ui| {
        let open = ui.memory().get::<CollapsingHeaderMemory>(container.id).open;
        let (header_response, _) = horizontal_fit_centered(ui, |ui| {
            let icon_text = if open {
                icons::CARET_DOWN
            } else {
                icons::CARET_RIGHT
            };
            icon(ui, icon_text);
            h_spacing(ui, 3.0);
        });
        ui.set_sense_mouse(header_response.node_ref, true);

        // Add the header now that we have access to the response data
        ui.with_parent(header_response.node_ref, |ui| {
            header(ui, &header_response);
        });

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

        header_response
    })
} 

pub fn collapsing_label<S: Into<String>, F: FnOnce(&mut UI)>(ui: &mut UI, label_text: S, contents: F) -> Response {
    let text_color = ui.style::<theme::TextColor>();
    collapsing_header(ui, |ui, response| {
        let label_response = label(ui, label_text);
        button_text_color_animation(ui, label_response.node_ref, response, text_color);
    }, contents) 
}
