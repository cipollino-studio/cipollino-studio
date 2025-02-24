
use crate::{Axis, PerAxis, Size, UINodeParams, UI};

use super::draggable_line;

pub fn resizable_panel<R, F: FnOnce(&mut UI) -> R>(ui: &mut UI, axis: Axis, size: &mut f32, contents: F) -> R {
    let result = ui.with_node(
        UINodeParams::new_per_axis(PerAxis::along_across(axis, Size::px(*size), Size::fr(1.0))),
        contents
    ).1;

    let divider_response = draggable_line(ui, axis.other()); 
    *size += divider_response.drag_delta(ui).on_axis(axis);

    result
}
