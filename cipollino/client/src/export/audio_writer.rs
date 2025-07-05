
use std::{io::Write, path::PathBuf, process::{Command, Stdio}};

use crate::{export::ffmpeg_path::FFMPEG_PATH, AudioPlaybackState};

pub fn audio_encoding_thread(path: PathBuf, audio_state: AudioPlaybackState, length: i64) {

    let mut process = Command::new(FFMPEG_PATH)
        .arg("-y") // Override output
        .arg("-f") // Input format
        .arg("s16le")
        .arg("-ac")
        .arg("2") // Number of output channels
        .arg("-i")
        .arg("-")
        .arg(path.to_str().unwrap())
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn().unwrap();
    let mut stdin = process.stdin.take().unwrap();
    let _stderr = process.stderr.take().unwrap();
    let _stdout = process.stdout.take().unwrap();
    let mut byte_buffer = Vec::new();

    for t in 0..length {
        let sample = audio_state.sample(t); 
        for c in 0..sample.len() {
            let sample = sample[c];
            let sample = (sample * (i16::MAX as f32)) as i16;
            byte_buffer.extend_from_slice(&sample.to_le_bytes());
        }
    }
    let _ = stdin.write_all(&byte_buffer);

    drop(stdin);

    process.wait().unwrap();

}
