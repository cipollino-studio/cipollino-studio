
use crate::{Margin, Size, UINodeParams, UI};

use super::{margin, Theme};

pub fn window<F: FnOnce(&mut UI)>(ui: &mut UI, contents: F) {
    let theme = ui.style::<Theme>();
    let fill = theme.bg_popup;
    let stroke = theme.widget_stroke();
    ui.with_node(
        UINodeParams::new(Size::fit(), Size::fit())
            .with_fill(fill)
            .with_stroke(stroke),
        |ui| {
            margin(ui, Margin::new(stroke.width, stroke.width, 0.0, 0.0), contents);
        } 
    );
}
