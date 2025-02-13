
use crate::{Layout, Margin, Response, Size, UINodeParams, UI};

use super::Theme;

pub fn container<R, F: FnOnce(&mut UI) -> R>(ui: &mut UI, width: Size, height: Size, layout: Layout, body: F) -> (Response, R) {
    ui.with_node(UINodeParams::new(width, height).with_layout(layout), body)
}

pub fn horizontal<R, F: FnOnce(&mut UI) -> R>(ui: &mut UI, body: F) -> (Response, R) {
    let container = ui.node(
        UINodeParams::new(Size::fr(1.0), Size::fit())
            .with_layout(Layout::horizontal())
    );
    (container, ui.with_parent(container.node_ref, body))
}

pub fn horizontal_fit<R, F: FnOnce(&mut UI) -> R>(ui: &mut UI, body: F) -> (Response, R) {
    let container = ui.node(
        UINodeParams::new(Size::fit(), Size::fit())
            .with_layout(Layout::horizontal())
    );
    (container, ui.with_parent(container.node_ref, body))
}

pub fn horizontal_fit_centered<R, F: FnOnce(&mut UI) -> R>(ui: &mut UI, body: F) -> (Response, R) {
    let container = ui.node(
        UINodeParams::new(Size::fit(), Size::fit())
            .with_layout(Layout::horizontal().align_center())
    );
    (container, ui.with_parent(container.node_ref, body))
}

pub fn vertical<R, F: FnOnce(&mut UI) -> R>(ui: &mut UI, body: F) -> (Response, R) {
    let container = ui.node(
        UINodeParams::new(Size::fr(1.0), Size::fit())
            .with_layout(Layout::vertical())
    );
    (container, ui.with_parent(container.node_ref, body))
}

pub fn vertical_fit<R, F: FnOnce(&mut UI) -> R>(ui: &mut UI, body: F) -> (Response, R) {
    let container = ui.node(
        UINodeParams::new(Size::fit(), Size::fit())
            .with_layout(Layout::vertical())
    );
    (container, ui.with_parent(container.node_ref, body))
}

pub fn vertical_fit_centered<R, F: FnOnce(&mut UI) -> R>(ui: &mut UI, body: F) -> (Response, R) {
    let container = ui.node(
        UINodeParams::new(Size::fit(), Size::fit())
            .with_layout(Layout::vertical().align_center())
    );
    (container, ui.with_parent(container.node_ref, body))
}

pub fn margin<R, F: FnOnce(&mut UI) -> R>(ui: &mut UI, body: F) -> R {
    let theme = ui.style::<Theme>();
    let widget_marin = theme.widget_margin;
    ui.with_node(UINodeParams::new(Size::fit(), Size::fit()).with_margin(Margin::same(widget_marin)), body).1
}
