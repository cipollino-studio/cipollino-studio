
use project::{Client, ClipInner, Layer, LayerParent, LayerPtr, Ptr};

use super::layers::LayerDropLocation;

pub(super) enum RenderLayerKind<'proj> {
    Layer(Ptr<Layer>, &'proj Layer)
}

pub(super) struct RenderLayer<'proj> {
    idx: usize,
    pub kind: RenderLayerKind<'proj>
}

pub(super) struct RenderList<'proj> {
    pub layers: Vec<RenderLayer<'proj>>
}

fn add_layers<'proj>(render_layers: &mut Vec<RenderLayer<'proj>>, layers: &'proj alisa::ChildList<LayerPtr>, client: &'proj Client) {
    for (idx, layer) in layers.iter().enumerate() {
        match layer {
            project::LayerPtr::Layer(layer_ptr) => {
                let layer_ptr = layer_ptr;
                if let Some(layer) = client.get(layer_ptr) {
                    render_layers.push(RenderLayer {
                        idx,
                        kind: RenderLayerKind::Layer(layer_ptr, layer)
                    });
                }
            },
        }
    }
}

impl<'proj> RenderList<'proj> {

    pub fn make(client: &'proj Client, clip: &'proj ClipInner) -> Self {
        let mut layers = Vec::new();

        add_layers(&mut layers, &clip.layers, &client);

        Self {
            layers
        }
    }

    pub fn iter<'list>(&'list self) -> impl Iterator<Item = &RenderLayer> + 'list {
        self.layers.iter()
    }

    pub fn len(&self) -> usize {
        self.layers.len()
    }

    pub fn get_transfer_location(&self, drop_location: LayerDropLocation) -> (LayerParent, usize) {
        let render_layer = &self.layers[drop_location.render_list_idx];
        match &render_layer.kind {
            RenderLayerKind::Layer(_ptr, layer) => {
                (layer.parent, render_layer.idx + if drop_location.above { 0 } else { 1 }) 
            }
        }
    }

}
