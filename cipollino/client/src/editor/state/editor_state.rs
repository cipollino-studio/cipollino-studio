
use std::{cell::RefCell, collections::HashMap};
use std::rc::Rc;
use project::{Clip, ClipInner, Layer, Ptr, Stroke};

use crate::{SelectTool, ToolDyn};

use crate::{Selection, SelectionKind};

use super::State;

pub struct EditorState {
    pub time: f32,
    pub playing: bool,

    pub open_clip: Ptr<Clip>,
    pub active_layer: Ptr<Layer>,

    pub curr_tool: Rc<RefCell<Box<dyn ToolDyn>>>,

    pub selection: Selection,

    pub stroke_mesh_cache: HashMap<Ptr<Stroke>, malvina::StrokeMesh>,
    pub stroke_preview: Option<malvina::StrokeMesh>,

    pub color: pierro::Color,

    windows_to_open: Vec<Box<dyn pierro::WindowDyn<Context = State>>>
}

impl EditorState {

    pub fn new() -> Self {
        Self {
            time: 0.0,
            playing: false,

            open_clip: Ptr::null(),
            active_layer: Ptr::null(),
            
            curr_tool: Rc::new(RefCell::new(Box::new(SelectTool::default()))),

            selection: Selection::new(),

            stroke_mesh_cache: HashMap::new(),
            stroke_preview: None,

            color: pierro::Color::BLACK,

            windows_to_open: Vec::new()
        }
    }

    pub fn jump_to(&mut self, time: f32) {
        self.time = time;
        self.playing = false;
        self.selection.clear();
    }

    pub fn tick_playback(&mut self, ui: &mut pierro::UI, clip: &ClipInner) {
        if self.playing {
            self.time += ui.input().delta_time;
            ui.request_redraw();
            if self.selection.kind() == SelectionKind::Scene {
                self.selection.clear();
            }
        }

        if self.time > clip.duration() {
            self.time = 0.0;
        }
        self.time = self.time.max(0.0);
    }

    pub fn open_clip(&mut self, clip_ptr: Ptr<Clip>) {
        self.open_clip = clip_ptr;
        self.jump_to(0.0);
    }

    pub fn open_window<W: pierro::Window<Context = State>>(&mut self, window: W) {
        self.windows_to_open.push(Box::new(window));
    }

    pub fn open_queued_windows(&mut self, windows: &mut pierro::WindowManager<State>) {
        for window in std::mem::replace(&mut self.windows_to_open, Vec::new()) {
            windows.open_window_dyn(window);
        }
    }

}
