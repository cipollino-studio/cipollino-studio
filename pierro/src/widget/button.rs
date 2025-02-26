
use crate::{Color, Response, Size, TextStyle, UINodeParams, UIRef, UI};

use super::{animate, icon, icon_text_style, theme::{self, hovered_color, label_text_style, pressed_color}};

pub fn button_color_animation(ui: &mut UI, interaction: &Response, base_color: Color) -> Color {
    let target_color = if interaction.mouse_down() {
        pressed_color(base_color)
    } else if interaction.hovered {
        hovered_color(base_color)
    } else {
        base_color
    };
    let rate = ui.style::<theme::ColorTransitionRate>(); 
    animate(ui, interaction.id, target_color, rate)
}

pub fn button_fill_animation(ui: &mut UI, node: UIRef, interaction: &Response, base_color: Color) {
    let color = button_color_animation(ui, interaction, base_color);
    ui.set_fill(node, color);
}

pub fn button_text_color_animation(ui: &mut UI, node: UIRef, interaction: &Response, base_color: Color) {
    let color = button_color_animation(ui, interaction, base_color);
    ui.set_text_color(node, color);
}

pub fn button_with_text_style<S: Into<String>>(ui: &mut UI, label: S, style: TextStyle) -> Response {
    let bg = ui.style::<theme::BgButton>();
    let margin = ui.style::<theme::WidgetMargin>();
    let rounding = ui.style::<theme::WidgetRounding>();

    let response = ui.node(
        UINodeParams::new(Size::text(), Size::text())
            .with_fill(bg)
            .with_margin(margin)
            .with_text(label)
            .with_text_style(style)
            .with_rounding(rounding)
            .sense_mouse()
    );

    button_fill_animation(ui, response.node_ref, &response, bg); 

    response
}

pub fn button<S: Into<String>>(ui: &mut UI, label: S) -> Response {
    let text_style = label_text_style(ui);
    button_with_text_style(ui, label, text_style)
}

pub fn icon_button<S: Into<String>>(ui: &mut UI, icon: S) -> Response {
    let text_style = icon_text_style(ui);
    button_with_text_style(ui, icon, text_style)
}

pub fn clickable_icon<S: Into<String>>(ui: &mut UI, icon_text: S) -> Response {
    let base_color = ui.style::<theme::TextColor>();
    let response = icon(ui, icon_text);
    ui.set_sense_mouse(response.node_ref, true);
    button_text_color_animation(ui, response.node_ref, &response, base_color);
    response
}
