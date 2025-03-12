
use std::{cell::RefCell, collections::HashMap};
use std::rc::Rc;
use project::{Client, Clip, ClipInner, Layer, Ptr, Stroke};

use crate::ToolDyn;

use super::Selection;

pub struct EditorState {
    pub time: f32,
    pub playing: bool,

    pub open_clip: Ptr<Clip>,
    pub active_layer: Ptr<Layer>,

    pub curr_tool: Rc<RefCell<Box<dyn ToolDyn>>>,

    pub selection: Selection,

    pub stroke_mesh_cache: HashMap<Ptr<Stroke>, malvina::StrokeMesh>,
    pub stroke_preview: Option<malvina::StrokeMesh> 
}

impl EditorState {

    pub fn jump_to(&mut self, time: f32) {
        self.time = time;
        self.playing = false;
    }

    pub(super) fn tick_playback(&mut self, ui: &mut pierro::UI, clip: &ClipInner) {
        if self.playing {
            self.time += ui.input().delta_time;
            ui.request_redraw();
        }

        if self.time > clip.duration() {
            self.time = 0.0;
        }
        self.time = self.time.max(0.0);
    }

    pub fn open_clip(&mut self, client: &Client, clip_ptr: Ptr<Clip>) {
        if client.get(clip_ptr).is_none() {
            return;
        } 
        self.open_clip = clip_ptr;
        self.jump_to(0.0);
    }

}
