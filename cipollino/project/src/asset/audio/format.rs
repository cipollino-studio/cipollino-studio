
#[derive(Clone, Copy, PartialEq, Eq, alisa::Serializable)]
pub enum AudioEncoding {
    /// Raw samples - little-endian i16, interleaved PCM samples
    Raw
}

#[derive(Clone, Copy, alisa::Serializable)]
pub struct AudioFormat {
    pub encoding: AudioEncoding,
    pub sample_rate: u32,
    pub n_channels: u32
}

impl Default for AudioFormat {

    fn default() -> Self {
        Self {
            encoding: AudioEncoding::Raw,
            sample_rate: 44100,
            n_channels: 2
        }
    }

}
