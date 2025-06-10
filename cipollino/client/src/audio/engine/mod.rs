
use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

mod state;
pub use state::*;

pub struct AudioEngine {
    sample_rate: u32,
    channels: u32,
    time: Arc<Mutex<i64>>,
    state: Arc<Mutex<AudioPlaybackState>>,
    stream: cpal::Stream
}

fn run_audio_thread<T: cpal::SizedSample + cpal::FromSample<f32>>(device: &cpal::Device, config: &cpal::StreamConfig, time: Arc<Mutex<i64>>, state: Arc<Mutex<AudioPlaybackState>>) -> Option<(cpal::Stream, u32, u32)> {
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let initial_time = *time.lock().unwrap();
            let state = state.lock().unwrap();

            for i in 0..(data.len() / channels) {
                let t = initial_time as usize + i;
                let sample = state.sample(t as i64);

                if channels == 1 {
                    data[i] = T::from_sample(0.5 * (sample[0] + sample[1]));
                } else {
                    for c in 0..2 {
                        data[channels * i + c] = T::from_sample(sample[c]);
                    }
                }
            }

            *time.lock().unwrap() = initial_time + (data.len() / channels) as i64;
        },
        |_| {},
        None,
    ).ok()?;

    Some((stream, sample_rate as u32, channels as u32))
}

impl AudioEngine {

    pub fn new() -> Option<Self> {
        let host = cpal::default_host();
        let device = host.default_output_device()?;
        let config = device.default_output_config().ok()?;

        let time = Arc::new(Mutex::new(0));
        let state = Arc::new(Mutex::new(AudioPlaybackState::new()));

        let (stream, sample_rate, channels) = match config.sample_format() {
            cpal::SampleFormat::I8  => run_audio_thread::<i8>(&device, &config.into(), time.clone(), state.clone()),
            cpal::SampleFormat::I16 => run_audio_thread::<i16>(&device, &config.into(), time.clone(), state.clone()),
            cpal::SampleFormat::I32 => run_audio_thread::<i32>(&device, &config.into(), time.clone(), state.clone()),
            cpal::SampleFormat::I64 => run_audio_thread::<i64>(&device, &config.into(), time.clone(), state.clone()),
            cpal::SampleFormat::U8  => run_audio_thread::<u8>(&device, &config.into(), time.clone(), state.clone()),
            cpal::SampleFormat::U16 => run_audio_thread::<u16>(&device, &config.into(), time.clone(), state.clone()),
            cpal::SampleFormat::U32 => run_audio_thread::<u32>(&device, &config.into(), time.clone(), state.clone()),
            cpal::SampleFormat::U64 => run_audio_thread::<u64>(&device, &config.into(), time.clone(), state.clone()),
            cpal::SampleFormat::F32 => run_audio_thread::<f32>(&device, &config.into(), time.clone(), state.clone()),
            cpal::SampleFormat::F64 => run_audio_thread::<f64>(&device, &config.into(), time.clone(), state.clone()),
            _ => return None
        }?;

        Some(Self {
            sample_rate,
            channels,
            time,
            state,
            stream
        })
    }

    pub fn play(&mut self) {
        let _ = self.stream.play();
    }

    pub fn pause(&mut self) {
        let _ = self.stream.pause();
    }

    pub fn time(&self) -> i64 {
        *self.time.lock().unwrap()
    }

    pub fn set_time(&mut self, time: i64) {
        *self.time.lock().unwrap() = time;
    }

    pub fn set_audio_state(&mut self, state: AudioPlaybackState) {
        *self.state.lock().unwrap() = state;
    } 

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn channels(&self) -> u32 { 
        self.channels
    }

}
