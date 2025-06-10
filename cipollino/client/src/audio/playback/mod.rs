
use crate::{AppSystems, EditorState, LayerRenderList, ProjectState, RenderLayerKind};

mod buffering;
mod state;

mod cache;
pub use cache::*;

impl EditorState {

    pub fn tick_audio_playback(&mut self, systems: &mut AppSystems, project: &ProjectState, layers: &LayerRenderList) {
        for layer in layers.iter() {
            match layer.kind {
                RenderLayerKind::AudioLayer(_, audio_layer) => {
                    self.buffer_audio_blocks(project, audio_layer);
                },
                _ => {}
            }
        }

        systems.audio.set_audio_state(self.construct_audio_state(project, layers, systems.audio.sample_rate()));
    }

}
