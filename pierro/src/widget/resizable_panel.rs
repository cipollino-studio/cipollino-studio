
use crate::{Axis, PerAxis, Size, UINodeParams, UI};

use super::draggable_line;

pub fn resizable_panel<F: FnOnce(&mut UI)>(ui: &mut UI, axis: Axis, size: &mut f32, contents: F) {
    ui.with_node(
        UINodeParams::new_per_axis(PerAxis::along_across(axis, Size::px(*size), Size::fr(1.0))),
        contents
    );

    let divider_response = draggable_line(ui, axis.other()); 
    *size += divider_response.drag_delta(ui).on_axis(axis);
}
