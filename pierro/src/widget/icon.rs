
use crate::{Response, Size, TextStyle, UINodeParams, UI};

use super::theme;

pub fn icon_text_style(ui: &mut UI) -> TextStyle {
    let color = ui.style::<theme::TextColor>();
    let font_size = ui.style::<theme::LabelFontSize>();
    TextStyle {
        color,
        font_size,
        line_height: 1.0,
        font: ui.icon_font(),
    }
}

pub fn icon<S: Into<String>>(ui: &mut UI, icon: S) -> Response {
    let text_style = icon_text_style(ui);

    ui.node(
        UINodeParams::new(Size::text(), Size::text())
            .with_text(icon)
            .with_text_style(text_style)
    )

}

