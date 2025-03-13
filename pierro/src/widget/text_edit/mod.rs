
use crate::{Response, Size, UINodeParams, UI};

use super::theme;

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
    ui.node(
        UINodeParams::new(Size::px(size), Size::px(font_size + widget_margin.v_total()))
            .sense_mouse()
            .sense_keyboard()
            .with_fill(color)
            .with_rounding(widget_rounding)
    )
}

pub fn text_edit(ui: &mut UI, text: &mut String) -> TextEditResponse {
    let text_edit = text_edit_base(ui, 200.0);

    if text_edit.mouse_pressed() && !text_edit.is_focused(ui) {
        text_edit.request_focus(ui);
    }

    text_edit_interaction(ui, text_edit, text) 
}
