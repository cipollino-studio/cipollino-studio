
use std::{io::Write, path::{Path, PathBuf}, process::{Command, Stdio}, sync::mpsc, thread};

use crate::export::ffmpeg_path::FFMPEG_PATH;

enum VideoWriterMessage {
    Frame(Vec<u8>),
    Close
}

pub struct VideoWriter {
    tx: mpsc::Sender<VideoWriterMessage>,
    thread: thread::JoinHandle<()>
}


impl VideoWriter {

    pub fn new(out: PathBuf, w: u32, h: u32, fps: f32, audio_path: &Path) -> Result<Self, String> {

        let out = out.with_extension("mp4");

        let (tx, rx) = mpsc::channel::<VideoWriterMessage>();

        let mut process = Command::new(FFMPEG_PATH)
            .arg("-y") // Override output
            .arg("-f") // Input format
            .arg("rawvideo")
            .arg("-pix_fmt")
            .arg("rgb24")
            .arg("-s")
            .arg(format!("{}x{}", w, h))
            .arg("-r")
            .arg(format!("{}", fps))
            .arg("-i")
            .arg("-")
            .arg("-i")
            .arg(audio_path)
            .arg("-c:v")
            .arg("libx264")
            .arg("-filter:v")
            .arg("scale=w=iw:h=ih:out_range=pc,format=yuv420p")
            .arg(out.to_str().unwrap())
            .stdin(Stdio::piped())
            .spawn().map_err(|err| err.to_string())?;
        let mut stdin = process.stdin.take().unwrap();

        let thread = thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                match msg {
                    VideoWriterMessage::Frame(frame) => {
                        stdin.write_all(frame.as_slice()).unwrap();
                    },
                    VideoWriterMessage::Close => break 
                }
            }
            drop(stdin); 
            process.wait().unwrap();
        });

        Ok(Self {
            tx,
            thread
        })
    }

    pub fn write_frame(&mut self, data: Vec<u8>) -> Result<(), String> {
        self.tx.send(VideoWriterMessage::Frame(data)).map_err(|err| err.to_string())
    }

    pub fn close(&mut self) -> Result<(), String> {
        self.tx.send(VideoWriterMessage::Close).map_err(|err| err.to_string())
    }

    pub fn done(&self) -> bool {
        self.thread.is_finished()
    }

}
