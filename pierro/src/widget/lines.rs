
use crate::{Axis, CursorIcon, Margin, PerAxis, Response, Size, UINodeParams, UI};

use super::{h_spacing, horizontal_fit, v_spacing, vertical_fit, theme};

fn line_params(ui: &mut UI, axis: Axis) -> UINodeParams {
    let stroke = ui.style::<theme::WidgetStroke>(); 
    UINodeParams::new_per_axis(PerAxis::along_across(axis, Size::fr(1.0), Size::px(stroke.width).no_shrink()))
        .with_fill(stroke.color)
}

pub fn h_line(ui: &mut UI) {
    let params = line_params(ui, Axis::X);
    ui.node(params);
}

pub fn v_line(ui: &mut UI) {
    let params = line_params(ui, Axis::Y);
    ui.node(params);
}

pub fn h_divider(ui: &mut UI) {
    let gap = ui.style::<theme::DividerLineGap>(); 
    horizontal_fit(ui, |ui| {
        h_spacing(ui, gap);
        h_line(ui);
        h_spacing(ui, gap);
    });
}

pub fn v_divider(ui: &mut UI) {
    let gap = ui.style::<theme::DividerLineGap>(); 
    vertical_fit(ui, |ui| {
        v_spacing(ui, gap);
        v_line(ui);
        v_spacing(ui, gap);
    });
}

const INTERACTION_MARGIN: Margin = Margin::same(5.0);

pub fn draggable_line(ui: &mut UI, axis: Axis) -> Response {
    let params = line_params(ui, axis)
        .with_interaction_margin(INTERACTION_MARGIN)
        .sense_mouse()
        .with_interaction_priority();
    let response = ui.node(params);
    if response.drag_started() {
        response.request_focus(ui);
    }
    if response.drag_stopped() {
        response.release_focus(ui);
    }
    if response.hovered || response.dragging() {
        ui.set_cursor(match axis {
            Axis::X => CursorIcon::NsResize,
            Axis::Y => CursorIcon::EwResize,
        });
    }
    response
}

pub fn h_draggable_line(ui: &mut UI) -> Response {
    draggable_line(ui, Axis::X)
}

pub fn v_draggable_line(ui: &mut UI) -> Response {
    draggable_line(ui, Axis::Y) 
}
