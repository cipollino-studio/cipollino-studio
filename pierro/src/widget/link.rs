
use crate::{Response, UI};

use super::{button_text_color_animation, h_spacing, horizontal_fit_centered, icon, label, theme};

pub fn link<S: Into<String>>(ui: &mut UI, text: S) -> Response {
    let link_color = ui.style::<theme::LinkColor>();
    let response = label(ui, text);
    ui.set_sense_mouse(response.node_ref, true);
    button_text_color_animation(ui, response.node_ref, &response, link_color);
    response
}

pub fn link_with_icon<S: Into<String>>(ui: &mut UI, text: S, icon_text: &'static str) -> Response {
    let link_color = ui.style::<theme::LinkColor>();
    let (response, (icon_node, link_node)) = horizontal_fit_centered(ui, |ui| {
        let icon_resp = icon(ui, icon_text);
        h_spacing(ui, 3.0);
        let link_resp = label(ui, text);
        (icon_resp.node_ref, link_resp.node_ref)
    });
    ui.set_sense_mouse(response.node_ref, true);
    button_text_color_animation(ui, icon_node, &response, link_color);
    button_text_color_animation(ui, link_node, &response, link_color);
    response
}
