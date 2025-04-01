
use crate::{Layout, Size, UINodeParams, UIRef, UI};

use super::{h_spacing, horizontal_fit, label, v_spacing};

pub struct KeyValueBuilder<'ui, 'a, 'b> {
    ui: &'ui mut UI<'a, 'b>,
    key_column: UIRef,
    value_column: UIRef
}

impl KeyValueBuilder<'_, '_, '_> {

    pub fn labeled_with_size<R, F: FnOnce(&mut UI) -> R>(&mut self, label_text: impl Into<String>, size: f32, value: F) -> R {
        self.ui.with_parent(self.key_column, |ui| {
            ui.with_node(
                UINodeParams::new(Size::fit(), Size::px(size))
                    .with_layout(Layout::horizontal().align_center()),
                |ui| {
                    label(ui, label_text);
                }
            );
        });
        self.ui.with_parent(self.value_column, |ui| {
            ui.with_node(
                UINodeParams::new(Size::fit(), Size::px(size))
                    .with_layout(Layout::horizontal().align_center()),
                |ui| {
                    value(ui)
                }
            ).1
        })
    }

    pub fn labeled<R, F: FnOnce(&mut UI) -> R>(&mut self, label_text: impl Into<String>, value: F) -> R {
        self.labeled_with_size(label_text, 25.0, value)
    }

    pub fn spacing(&mut self, spacing: f32) {
        self.ui.with_parent(self.key_column, |ui| {
            v_spacing(ui, spacing);
        });
        self.ui.with_parent(self.value_column, |ui| {
            v_spacing(ui, spacing);
        });
    }

}

pub fn key_value_layout<R, F: FnOnce(&mut KeyValueBuilder) -> R>(ui: &mut UI, contents: F) -> R {
    horizontal_fit(ui, |ui| {
        let key_column = ui.node(
            UINodeParams::new(Size::fit(), Size::fit())
                .with_layout(Layout::vertical().align_max())
        ).node_ref;
        h_spacing(ui, 3.0);
        let value_column = ui.node(
            UINodeParams::new(Size::fit(), Size::fit())
                .with_layout(Layout::vertical())
        ).node_ref;

        let mut builder = KeyValueBuilder {
            ui,
            key_column,
            value_column
        };

        contents(&mut builder)
    }).1
}
