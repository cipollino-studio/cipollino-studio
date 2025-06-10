

use std::path::PathBuf;
use std::fs::File;

use alisa::Ptr;
use project::{Action, AddBlockToAudioClip, AudioClip, AudioClipTreeData, AudioEncoding, AudioFormat, Client, CreateAudioClip};
use symphonia::core::audio::AudioBufferRef;
use symphonia::core::{audio::SampleBuffer, formats::FormatReader};
use symphonia::core::codecs::{Decoder, DecoderOptions};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::{EditorState, PanelContext, Window};

fn next_packet_buffer<'decoder>(format: &mut Box<dyn FormatReader>, decoder: &'decoder mut Box<dyn Decoder>) -> Option<AudioBufferRef<'decoder>> {
    let packet = format.next_packet().ok()?;
    decoder.decode(&packet).ok()
}

pub struct AudioImportWindow {
    format: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    sample_buf: SampleBuffer<i16>,
    clip: Ptr<AudioClip>,
    samples: Vec<i16>,
    n_channels: u32,

    samples_in_file: Option<u64>,
    samples_loaded: u64
}

impl AudioImportWindow {

    pub fn open(client: &Client, editor: &mut EditorState, path: PathBuf) {

        // Open the audio file
        let Ok(file) = File::open(&path) else { return; };

        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        if let Some(ext) = path.extension() {
            hint.with_extension(ext.to_string_lossy().to_string().as_str());
        }

        let format_opts: FormatOptions = Default::default();
        let metadata_opts: MetadataOptions = Default::default();
        let decoder_opts: DecoderOptions = Default::default();

        let probed = symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts);
        let Ok(probed) = probed else { return; };

        let mut format: Box<dyn FormatReader + 'static> = probed.format; 

        let Some(track) = format.default_track() else { return; };
        let samples_in_file = track.codec_params.n_frames;

        let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts).unwrap();

        // Get the first pack to find out sample rate and the number of channels 
        let Some(buf) = next_packet_buffer(&mut format, &mut decoder) else { return; };
        let spec = *buf.spec();
        let sample_rate = spec.rate;
        let n_channels = spec.channels.count() as u32;

        // Create sample buffer
        let sample_buffer_capacity = buf.capacity();
        let mut sample_buf = SampleBuffer::<i16>::new(sample_buffer_capacity as u64, spec);

        // Read first few samples
        sample_buf.copy_interleaved_ref(buf);
        let samples = sample_buf.samples().into();

        // Create audio clip
        let name = path.file_stem().map(|name| name.to_string_lossy().to_string()).unwrap_or("Audio".to_string());
        let clip = client.next_ptr();
        client.queue_action(Action::single(editor.action_context("Create Audio Clip"), CreateAudioClip {
            ptr: clip,
            parent: Ptr::null(),
            data: AudioClipTreeData {
                name,
                format: AudioFormat {
                    encoding: AudioEncoding::Raw,
                    sample_rate,
                    n_channels
                },
                length: 0,
                blocks: Vec::new(),
            },
        }));

        editor.open_window(Self {
            format,
            decoder,
            sample_buf,
            clip,
            samples,
            n_channels,

            samples_in_file,
            samples_loaded: 0
        }); 
    }

}

// NOTE: this is the number of samples on a single channel
const MAX_SAMPLES_PER_BLOCK: usize = 100_000;

impl AudioImportWindow {

    fn add_block(&mut self, context: &mut PanelContext) {
        let block_data = &self.samples[0..self.samples.len().min(MAX_SAMPLES_PER_BLOCK)];

        // Reencode block in the "raw" format
        let mut block_bytes = vec![0u8; block_data.len() * 2].into_boxed_slice();
        for i in 0..block_data.len() {
            block_bytes[i * 2 + 0] = block_data[i].to_le_bytes()[0];
            block_bytes[i * 2 + 1] = block_data[i].to_le_bytes()[1];
        }
        let block_length = block_data.len() / (self.n_channels as usize);

        context.project.client.queue_operation(AddBlockToAudioClip {
            ptr: context.project.client.next_ptr(),
            clip: self.clip,
            length: block_length,
            data: block_bytes,
        });
    }

    fn tick_import(&mut self, close: &mut bool, context: &mut PanelContext) {
        let Some(buf) = next_packet_buffer(&mut self.format, &mut self.decoder) else {
            self.add_block(context); 
            self.samples.clear();
            *close = true;
            return;
        };
        self.sample_buf.copy_interleaved_ref(buf);
        self.samples.extend_from_slice(self.sample_buf.samples()); 
        self.samples_loaded += (self.sample_buf.samples().len() as u64) / (self.n_channels as u64);

        while self.samples.len() >= MAX_SAMPLES_PER_BLOCK {
            self.add_block(context); 
            self.samples.drain(0..MAX_SAMPLES_PER_BLOCK);
        }
    }

}

impl Window for AudioImportWindow {

    fn title(&self) -> String {
        "Importing Audio...".to_string()
    }

    fn render<'ctx>(&mut self, ui: &mut pierro::UI, close: &mut bool, context: &mut PanelContext<'ctx>) {
        ui.request_redraw();
        for _ in 0..15 {
            self.tick_import(close, context);
        }

        if let Some(total_samples) = self.samples_in_file {
            let progress = (self.samples_loaded as f32) / (total_samples as f32);
            pierro::progress_bar(ui, progress);
        } else {
            // Highly unlikely, but let's have a fallback
            pierro::label(ui, "Loading...");
        }
    }

}
