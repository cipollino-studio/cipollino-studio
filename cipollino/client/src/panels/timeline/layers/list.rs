
use std::collections::HashSet;

use project::{Action, Client, Layer, LayerParent, Ptr, TransferLayer};

use super::LayerUI;

#[derive(Default, Clone, Debug)]
pub struct LayerList {
    pub layers: HashSet<Ptr<Layer>>
}

impl LayerList {

    pub fn add<L: LayerUI>(&mut self, ptr: Ptr<L>) {
        L::selection_list_mut(self).insert(ptr);
    }

    pub fn single<L: LayerUI>(ptr: Ptr<L>) -> Self {
        let mut selection = Self::default();
        selection.add(ptr);
        selection
    }

    fn render_contents_of_layer<L: LayerUI>(&self, ui: &mut pierro::UI, client: &Client) {
        for layer_ptr in L::selection_list(self).iter() {
            let Some(layer) = client.get(*layer_ptr) else { continue; };
            pierro::horizontal_fit_centered(ui, |ui| {
                pierro::icon(ui, L::ICON);
                pierro::label(ui, layer.name());
            });
        }
    }

    pub fn render_contents(&self, ui: &mut pierro::UI, client: &Client) {
        self.render_contents_of_layer::<Layer>(ui, client);
    }

    pub fn transfer(&self, action: &mut Action, new_parent: LayerParent, new_idx: usize) {
        for layer in &self.layers {
            action.push(TransferLayer {
                ptr: *layer,
                new_parent,
                new_idx,
            });
        }
    }

}
