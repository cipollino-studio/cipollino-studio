
use crate::app::prefs::UserPrefs;
use super::{keybind::{Keybind, PlayKeybind}, EditorState};

impl EditorState {

    pub fn playback_sec(&self) -> f64 {
        (self.playback_time as f64)  / (self.project.sample_rate as f64)
    }

    pub fn playback_frame(&self) -> i32 {
        (self.playback_sec() * (self.project.fps as f64)) as i32
    }

    pub fn update_playback(&mut self, dt: f32, ctx: &egui::Context) {
        if self.playing {
            self.playback_time += (dt * self.project.sample_rate) as i64; 
            ctx.request_repaint();
        }

        self.playback_time = self.playback_time.max(0);
        if let Some(clip) = self.project.clips.get(self.open_clip) {
            if self.playback_time as f32 > (*clip.length.value() as f32) / self.project.fps * self.project.sample_rate {
                self.playback_time = 0;
            }
        }
    } 

    pub fn playback_shortcuts(&mut self, ctx: &egui::Context, prefs: &mut UserPrefs) {
        if PlayKeybind::consume(ctx, prefs) {
            self.playing = !self.playing;
        }
    }

}
