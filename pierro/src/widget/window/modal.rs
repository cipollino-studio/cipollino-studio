
use crate::{Color, UI};

use crate::{centered, window};

pub fn modal<R, F: FnOnce(&mut UI) -> R>(ui: &mut UI, contents: F) -> R {
    let (layer, (_, inner)) = ui.layer(|ui| {
        centered(ui, |ui| {
            window(ui, contents)
        })
    });

    ui.set_sense_mouse(layer, true);
    ui.set_sense_scroll(layer, true);
    ui.set_sense_dnd_hover(layer, true);
    ui.set_fill(layer, Color::rgba(0.0, 0.0, 0.0, 0.3));

    inner
}
