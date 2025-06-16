
use project::AudioLayer;

use crate::{EditorState, ProjectState};

impl EditorState {

    pub(super) fn buffer_audio_blocks(&mut self, project: &ProjectState, layer: &AudioLayer) {
        // Load 5 seconds in advance
        let buffer_range = elic::Range::min_size(self.time, 5.0);

        for audio in layer.audio_instances.iter() {
            let Some(audio) = project.client.get(audio.ptr()) else { continue; };
            let Some(clip) = project.client.get(audio.clip) else { continue; };
            let audio_range = elic::Range::new(audio.start, audio.end);
            if !buffer_range.intersects(audio_range) {
                continue;
            }

            let intersection = buffer_range.intersect(audio_range);
            let samples = intersection.shift(-audio.start + audio.offset) * (clip.format.sample_rate as f32);

            let mut offset = 0;
            for (block_size, block_ptr) in &clip.blocks {
                let block_range = elic::Range::min_size(offset as f32, *block_size as f32);
                if block_range.intersects(samples) && project.client.get_ref(*block_ptr).is_none() {
                    project.client.request_load(*block_ptr);
                    return;
                }
                offset += *block_size;
            }
        }
    }

}
