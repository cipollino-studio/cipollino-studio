
use std::sync::Arc;

use crate::SampleBlock;

pub struct AudioPlaybackClip {
    pub begin: i64,
    pub end: i64,
    pub data: Arc<SampleBlock>
}

pub struct AudioPlaybackState {
    pub clips: Vec<AudioPlaybackClip>
}

impl AudioPlaybackState {

    pub fn new() -> Self {
        Self {
            clips: Vec::new()
        }
    }    

    pub fn sample(&self, t: i64) -> [f32; 2] {
        let mut sample = [0.0; 2];

        for clip in &self.clips {
            let clip_t = t - clip.begin;
            if clip_t < 0 || clip_t >= clip.data.samples.len() as i64 {
                continue;
            }
            for c in 0..2 {
                sample[c] += (clip.data.samples[clip_t as usize][c] as f32) / (i16::MIN as f32);
            }
        }

        sample
    }

}
