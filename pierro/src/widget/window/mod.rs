
use crate::{Margin, Response, Size, UINodeParams, UI};

use super::theme;

mod modal;
pub use modal::*;

mod manager;
pub use manager::*;

pub fn window<R, F: FnOnce(&mut UI) -> R>(ui: &mut UI, contents: F) -> (Response, R) {
    let fill = ui.style::<theme::BgPopup>();
    let stroke = ui.style::<theme::WidgetStroke>();
    ui.with_node(
        UINodeParams::new(Size::fit(), Size::fit())
            .with_fill(fill)
            .with_stroke(stroke)
            .sense_mouse()
            .with_margin(Margin::same(stroke.width)),
        contents 
    )
}
