
use std::{collections::HashMap, sync::Arc};

use alisa::Ptr;
use project::{AudioBlock, AudioEncoding, AudioFormat};

pub struct SampleBlock {
    pub samples: Vec<[i16; 2]>,
    pub volume: Vec<f32>
}

pub struct AudioBlockCache {
    sample_rate: u32,
    blocks: HashMap<Ptr<AudioBlock>, Arc<SampleBlock>>
}

impl AudioBlockCache {

    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            blocks: HashMap::new()
        }
    }

    fn decode_raw(input_channels: u32, data: &[u8], n_channels: u32, input_samples: &mut Vec<Vec<f32>>) {
        for t in (0..data.len()).step_by(2 * input_channels as usize) {
            for c in 0..(n_channels as usize) {
                let sample = i16::from_le_bytes([data[t + 2 * c + 0], data[t + 2 * c + 1]]);
                // Technically not a correct mapping, because i16::MAX doesn't get mapped to 1.0.
                let sample = (sample as f32) / (i16::MIN as f32);
                input_samples[c].push(sample);
            }

            // Map mono audio to stereo
            if n_channels == 1 {
                let sample = *input_samples[0].last().unwrap();
                input_samples[1].push(sample);
            }
        }
    }

    fn resample(input_samples: Vec<Vec<f32>>, input_sample_rate: u32, output_sample_rate: u32) -> Vec<Vec<f32>> {
        // World's dumbest linear resampler lol
        let resample_ratio = (output_sample_rate as f32) / (input_sample_rate as f32);
        let output_len = (resample_ratio * (input_samples[0].len() as f32)).round() as usize;
        let mut samples = vec![Vec::new(), Vec::new()];
        for t in 0..output_len {
            let input_t = (t as f32) / resample_ratio;
            let t0 = input_t.floor() as usize;
            let t1 = t0 + 1;
            let interp = input_t.fract();
            for c in 0..2 {
                let s0 = input_samples[c][t0.min(input_samples[c].len() - 1)];
                let s1 = input_samples[c][t1.min(input_samples[c].len() - 1)];
                samples[c].push(interp * s1 + (1.0 - interp) * s0);
            }
        }

        samples
    }

    pub fn get_samples(
        &mut self,
        format: AudioFormat,
        block_ptr: Ptr<AudioBlock>,
        block: &AudioBlock
    ) -> Arc<SampleBlock> {

        if let Some(block) = self.blocks.get(&block_ptr) {
            return block.clone();
        }

        // Decode samples from audio block
        let n_channels = format.n_channels.min(2);
        let mut input_samples = vec![Vec::new(), Vec::new()];
        match format.encoding {
            AudioEncoding::Raw => Self::decode_raw(format.n_channels, &block.data, n_channels, &mut input_samples),
        }

        // Resample the audio block to match sample rate
        let output_samples = if format.sample_rate == self.sample_rate {
            input_samples
        } else {
            Self::resample(input_samples, format.sample_rate, self.sample_rate)
        };
        assert_eq!(output_samples[0].len(), output_samples[1].len());
        let len = output_samples[0].len();

        // Calculate volume preview
        let mut volume = Vec::new();
        let volume_sample_window = 200;
        for t in (0..len).step_by(volume_sample_window) {
            let mut sample = 0.0;
            let n_samples = (len - t).min(volume_sample_window);
            for i in 0..n_samples {
                for c in 0..output_samples.len() {
                    sample += output_samples[c][t + i].abs();
                } 
            }
            sample /= (output_samples.len() * n_samples) as f32;
            volume.push(sample);
        }

        // Convert samples into final [i16; 2] form
        let mut samples = Vec::new();
        for t in 0..len {
            let mut full_sample = [0; 2]; 
            for c in 0..2 {
                let sample = (output_samples[c][t] * (i16::MAX as f32)).round() as i16;
                full_sample[c] = sample;
            }
            samples.push(full_sample);
        }

        let sample_block = SampleBlock {
            samples,
            volume
        };
        let sample_block = Arc::new(sample_block);
        self.blocks.insert(block_ptr, sample_block.clone());

        sample_block
    }

}
