
use project::{Client, ClipInner, Layer, LayerPtr, Ptr};

pub enum RenderLayerKind<'proj> {
    Layer(Ptr<Layer>, &'proj Layer)
}

pub struct RenderLayer<'proj> {
    pub idx: usize,
    pub kind: RenderLayerKind<'proj>
}

impl RenderLayer<'_> {

    pub fn any_ptr(&self) -> alisa::AnyPtr {
        match self.kind {
            RenderLayerKind::Layer(ptr, _) => ptr.any(),
        }
    }

}

/// List of all the layer-like things in a clip, ordered by how they appear in the timeline panel
pub struct LayerRenderList<'proj> {
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

impl<'proj> LayerRenderList<'proj> {

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

}
