
use crate::{CursorIcon, Response, Size, UINodeParams, UI};

use super::{button_fill_animation, theme};

mod keyboard;
use keyboard::*;

mod mouse;
use mouse::*;

mod paint;
use paint::*;

mod interaction;
pub use interaction::*;

pub fn text_edit_base(ui: &mut UI, size: f32) -> Response {
    let color = ui.style::<theme::BgTextField>(); 
    let widget_margin = ui.style::<theme::WidgetMargin>(); 
    let widget_rounding = ui.style::<theme::WidgetRounding>(); 
    let font_size = ui.style::<theme::LabelFontSize>();
    let response = ui.node(
        UINodeParams::new(Size::px(size), Size::px(font_size + widget_margin.v_total()))
            .sense_mouse()
            .sense_keyboard()
            .with_fill(color)
            .with_rounding(widget_rounding)
    );
    button_fill_animation(ui, response.node_ref, &response, color);
    response
}

pub fn text_edit(ui: &mut UI, text: &mut String) -> TextEditResponse {
    let text_edit = text_edit_base(ui, 200.0);

    if text_edit.mouse_pressed() && !text_edit.is_focused(ui) {
        text_edit_begin_editing(ui, &text_edit, text);
    }

    if text_edit.hovered && text_edit.contains_mouse(ui) {
        ui.set_cursor(CursorIcon::Text);
    }

    text_edit_interaction(ui, text_edit, text) 
}
