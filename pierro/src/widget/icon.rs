
use crate::{icons, Color, Response, Size, TextStyle, UINodeParams, UI};

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

/// A gap the size of an icon. Useful for aligning things
pub fn icon_gap(ui: &mut UI) -> Response {
    let text_style = TextStyle {
        color: Color::TRANSPARENT,
        ..icon_text_style(ui)
    };

    ui.node(
        UINodeParams::new(Size::text(), Size::text())
            .with_text(icons::ACORN)
            .with_text_style(text_style)
    )

}
