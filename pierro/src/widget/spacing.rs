
use crate::{Size, UINodeParams, UI};

pub fn h_spacing(ui: &mut UI, amount: f32) {
    ui.node(UINodeParams::new(Size::px(amount), Size::px(0.0)));
}

pub fn v_spacing(ui: &mut UI, amount: f32) {
    ui.node(UINodeParams::new(Size::px(0.0), Size::px(amount)));
}
