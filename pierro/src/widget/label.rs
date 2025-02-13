
use crate::{Response, Size, UINodeParams, UI};

use super::label_text_style;

pub fn label<S: Into<String>>(ui: &mut UI, label: S) -> Response {
    let text_style = label_text_style(ui);

    ui.node(
        UINodeParams::new(Size::text(), Size::text())
            .with_text(label)
            .with_text_style(text_style)
    )

}
