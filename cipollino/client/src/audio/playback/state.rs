
use alisa::Ptr;
use project::{AudioInstance, AudioLayer};

use crate::{AudioPlaybackClip, AudioPlaybackState, EditorState, LayerRenderList, ProjectState, RenderLayerKind};

impl EditorState {

    fn add_audio_instance_clips(&mut self, project: &ProjectState, audio: &AudioInstance, sample_rate: u32, clips: &mut Vec<AudioPlaybackClip>) {
        let Some(clip) = project.client.get(audio.clip) else { return; };

        let mut t = ((audio.start - audio.offset) * (sample_rate as f32)).round() as i64;
        let instance_start = (audio.start * (sample_rate as f32)).round() as i64; 
        let instance_end = (audio.end * (sample_rate as f32)).round() as i64; 
        for (block_size, block_ptr) in clip.blocks.iter() {
            let Some(block) = project.client.get(*block_ptr) else {
                t += ((*block_size as f32) * (sample_rate as f32) / (clip.format.sample_rate as f32)).round() as i64;
                continue;
            };
            let data = self.audio_cache.get_samples(clip.format, *block_ptr, block);
            let len = data.samples.len() as i64;

            if t + len > instance_start {
                clips.push(AudioPlaybackClip {
                    begin: t,
                    end: (t + len).min(instance_end),
                    offset: (instance_start - t).max(0),
                    data
                });
            }

            if t + len >= instance_end {
                break;
            }

            t += len;
        }

    }

    fn add_layer_clips(&mut self, project: &ProjectState, layer_ptr: Ptr<AudioLayer>, layer: &AudioLayer, sample_rate: u32, clips: &mut Vec<AudioPlaybackClip>) {
        if self.muted_layers.contains(&layer_ptr) {
            return;
        }
        for audio in layer.audio_instances.iter() {
            let Some(audio) = project.client.get(audio.ptr()) else { continue; };
            self.add_audio_instance_clips(project, audio, sample_rate, clips);
        }
    }

    pub(super) fn construct_audio_state(&mut self, project: &ProjectState, layers: &LayerRenderList, sample_rate: u32) -> AudioPlaybackState {
        let mut clips = Vec::new();
        for layer in layers.iter() {
            match layer.kind {
                RenderLayerKind::AudioLayer(audio_layer_ptr, audio_layer) => {
                    self.add_layer_clips(project, audio_layer_ptr, audio_layer, sample_rate, &mut clips);
                },
                _ => {}
            }
        }

        AudioPlaybackState {
            clips
        }
    }

}
