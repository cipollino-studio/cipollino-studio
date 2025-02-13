
use crate::{Response, Size, TextStyle, UINodeParams, UI};
use super::Theme;

pub fn icon_text_style(ui: &mut UI) -> TextStyle {
    let theme = ui.style::<Theme>(); 
    TextStyle {
        color: theme.text,
        font_size: theme.label_font_size,
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

