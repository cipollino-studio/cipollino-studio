
use project::{AudioLayer, Client, ClipInner, Layer, LayerGroup, LayerPtr, Ptr};

use super::EditorState;

pub enum RenderLayerKind<'proj> {
    Layer(Ptr<Layer>, &'proj Layer),
    AudioLayer(Ptr<AudioLayer>, &'proj AudioLayer),
    LayerGroup(Ptr<LayerGroup>, &'proj LayerGroup)
}

pub struct RenderLayer<'proj> {
    pub idx: usize,
    // How many layer groups is this layer in?
    pub depth: i32,
    pub kind: RenderLayerKind<'proj>
}

impl RenderLayer<'_> {

    pub fn any_ptr(&self) -> alisa::AnyPtr {
        match self.kind {
            RenderLayerKind::Layer(ptr, _) => ptr.any(),
            RenderLayerKind::AudioLayer(ptr, _) => ptr.any(),
            RenderLayerKind::LayerGroup(ptr, _) => ptr.any()
        }
    }

}

/// List of all the layer-like things in a clip, ordered by how they appear in the timeline panel
pub struct LayerRenderList<'proj> {
    pub layers: Vec<RenderLayer<'proj>>
}

fn add_layers<'proj>(render_layers: &mut Vec<RenderLayer<'proj>>, layers: &'proj alisa::ChildList<LayerPtr>, client: &'proj Client, editor: &EditorState, depth: i32) {
    for (idx, layer) in layers.iter().enumerate() {
        match layer {
            LayerPtr::Layer(layer_ptr) => {
                if let Some(layer) = client.get(layer_ptr) {
                    render_layers.push(RenderLayer {
                        idx,
                        depth,
                        kind: RenderLayerKind::Layer(layer_ptr, layer)
                    });
                }
            },
            LayerPtr::AudioLayer(audio_layer_ptr) => {
                if let Some(audio_layer) = client.get(audio_layer_ptr) {
                    render_layers.push(RenderLayer {
                        idx,
                        depth,
                        kind: RenderLayerKind::AudioLayer(audio_layer_ptr, audio_layer) 
                    });
                }
            },
            LayerPtr::LayerGroup(layer_group_ptr) => {
                if let Some(layer_group) = client.get(layer_group_ptr) {
                    render_layers.push(RenderLayer {
                        idx,
                        depth,
                        kind: RenderLayerKind::LayerGroup(layer_group_ptr, layer_group) 
                    });
                    if editor.open_layer_groups.contains(&layer_group_ptr) {
                        add_layers(render_layers, &layer_group.layers, client, editor, depth + 1);
                    }
                }
            }
        }
    }
}

impl<'proj> LayerRenderList<'proj> {

    pub fn make(client: &'proj Client, editor: &EditorState, clip: &'proj ClipInner) -> Self {
        let mut layers = Vec::new();

        add_layers(&mut layers, &clip.layers, &client, editor, 0);

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
