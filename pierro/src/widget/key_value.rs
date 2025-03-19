
use crate::{Layout, Size, UINodeParams, UIRef, UI};

use super::{h_spacing, horizontal, label};

pub struct KeyValueBuilder<'ui, 'a, 'b> {
    ui: &'ui mut UI<'a, 'b>,
    key_column: UIRef,
    value_column: UIRef
}

impl KeyValueBuilder<'_, '_, '_> {

    pub fn labeled_with_size<F: FnOnce(&mut UI)>(&mut self, label_text: impl Into<String>, size: f32, value: F) {
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
                    value(ui);
                }
            );
        });
    }

    pub fn labeled<F: FnOnce(&mut UI)>(&mut self, label_text: impl Into<String>, value: F) {
        self.labeled_with_size(label_text, 25.0, value);
    }

}

pub fn key_value_layout<F: FnOnce(&mut KeyValueBuilder)>(ui: &mut UI, contents: F) {
    horizontal(ui, |ui| {
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

        contents(&mut builder);
    });
}
