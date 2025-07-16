
use std::collections::HashSet;

use alisa::Ptr;
use project::{AudioInstance, AudioLayer};

use crate::{AudioBlockCache, AudioPlaybackClip, AudioPlaybackState, EditorState, LayerRenderList, ProjectState, RenderLayerKind};

impl AudioPlaybackState {
    
    fn add_audio_instance_clips(project: &ProjectState, audio: &AudioInstance, sample_rate: u32, clips: &mut Vec<AudioPlaybackClip>, audio_cache: &mut AudioBlockCache) {
        let Some(clip) = project.client.get(audio.clip) else { return; };

        let mut t = ((audio.start - audio.offset) * (sample_rate as f32)).round() as i64;
        let instance_start = (audio.start * (sample_rate as f32)).round() as i64; 
        let instance_end = (audio.end * (sample_rate as f32)).round() as i64; 
        for (block_size, block_ptr) in clip.blocks.iter() {
            let Some(block) = project.client.get(*block_ptr) else {
                t += ((*block_size as f32) * (sample_rate as f32) / (clip.format.sample_rate as f32)).round() as i64;
                continue;
            };
            let data = audio_cache.get_samples(clip.format, block_ptr.ptr(), block);
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

    fn add_layer_clips(project: &ProjectState, layer_ptr: Ptr<AudioLayer>, layer: &AudioLayer, sample_rate: u32, clips: &mut Vec<AudioPlaybackClip>, audio_cache: &mut AudioBlockCache, muted_layers: &HashSet<Ptr<AudioLayer>>) {
        if muted_layers.contains(&layer_ptr) {
            return;
        }
        for audio in layer.audio_instances.iter() {
            let Some(audio) = project.client.get(audio) else { continue; };
            Self::add_audio_instance_clips(project, audio, sample_rate, clips, audio_cache);
        }
    }

    pub fn construct(project: &ProjectState, layers: &LayerRenderList, sample_rate: u32, audio_cache: &mut AudioBlockCache, muted_layers: &HashSet<Ptr<AudioLayer>>) -> Self {
        let mut clips = Vec::new();
        for layer in layers.iter() {
            match layer.kind {
                RenderLayerKind::AudioLayer(audio_layer_ptr, audio_layer) => {
                    Self::add_layer_clips(project, audio_layer_ptr, audio_layer, sample_rate, &mut clips, audio_cache, muted_layers);
                },
                _ => {}
            }
        }

        AudioPlaybackState {
            clips
        }
    }

}

impl EditorState {

    pub fn construct_audio_state(&mut self, project: &ProjectState, layers: &LayerRenderList, sample_rate: u32) -> AudioPlaybackState {
        AudioPlaybackState::construct(project, layers, sample_rate, &mut self.audio_cache, &self.muted_layers)
    }

}
