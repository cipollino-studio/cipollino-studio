
use crate::{Margin, Size, UINodeParams, UI};

use super::{margin, theme};

pub fn window<F: FnOnce(&mut UI)>(ui: &mut UI, contents: F) {
    let fill = ui.style::<theme::BgPopup>();
    let stroke = ui.style::<theme::WidgetStroke>();
    ui.with_node(
        UINodeParams::new(Size::fit(), Size::fit())
            .with_fill(fill)
            .with_stroke(stroke),
        |ui| {
            margin(ui, Margin::new(stroke.width, stroke.width, 0.0, 0.0), contents);
        } 
    );
}
