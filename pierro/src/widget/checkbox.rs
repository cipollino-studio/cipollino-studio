
use crate::{icons, Color, Margin, Response, UI};

use super::{button_with_text_style, h_spacing, horizontal_fit, icon_text_style, label};

pub fn checkbox(ui: &mut UI, value: &mut bool) -> Response {
    let mut text_style = icon_text_style(ui);
    if !*value {
        text_style.color = Color::TRANSPARENT;
    }
    let response = button_with_text_style(ui, icons::CHECK, text_style);
    ui.set_margin(response.node_ref, Margin::ZERO);
    if response.mouse_clicked() {
        *value = !*value;
    }
    response
}

pub fn checkbox_labeled<S: Into<String>>(ui: &mut UI, label_text: S, value: &mut bool) -> Response {
    horizontal_fit(ui, |ui| {
        let response = checkbox(ui, value);
        h_spacing(ui, 5.0);
        label(ui, label_text);
        response
    }).1
}
