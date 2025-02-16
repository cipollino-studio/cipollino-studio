
use crate::{Margin, Size, UINodeParams, UI};

use super::Theme;

pub fn window<F: FnOnce(&mut UI)>(ui: &mut UI, contents: F) {
    let theme = ui.style::<Theme>();
    let fill = theme.bg_popup;
    let stroke = theme.widget_stroke();
    let margin = Margin::same(theme.window_margin);
    ui.with_node(
        UINodeParams::new(Size::fit(), Size::fit())
            .with_fill(fill)
            .with_stroke(stroke)
            .with_margin(margin),
        contents
    );
}
