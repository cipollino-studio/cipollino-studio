
use crate::UI;

use super::{centered, window};

pub fn modal<F: FnOnce(&mut UI)>(ui: &mut UI, contents: F) {
    ui.layer(|ui| {
        centered(ui, |ui| {
            window(ui, contents);
        });
    });
}
