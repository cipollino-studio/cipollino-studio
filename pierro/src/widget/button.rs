
use crate::{Color, Margin, Response, Size, TextStyle, UINodeParams, UIRef, UI};

use super::{animate, icon_text_style, label_text_style, Theme};

pub fn button_fill_animation(ui: &mut UI, node_to_fill: UIRef, interaction: &Response, base_color: Color) {
    let theme = ui.style::<Theme>();
    let target_color = if interaction.mouse_down() {
        theme.pressed_color(base_color)
    } else if interaction.hovered {
        theme.hovered_color(base_color)
    } else {
        base_color
    };
    let rate = theme.color_transition_animation_rate;
    let color = animate(ui, interaction.id, target_color, rate);
    ui.set_fill(node_to_fill, color);
}

pub fn button_with_text_style<S: Into<String>>(ui: &mut UI, label: S, style: TextStyle) -> Response {
    let theme = ui.style::<Theme>();
    let bg = theme.bg_button;
    let margin = theme.widget_margin;
    let rounding = theme.widget_rounding;

    let response = ui.node(
        UINodeParams::new(Size::text(), Size::text())
            .with_fill(bg)
            .with_margin(Margin::same(margin))
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
