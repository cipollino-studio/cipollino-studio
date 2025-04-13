
use std::collections::HashSet;
use std::{cell::RefCell, collections::HashMap};
use std::rc::Rc;
use project::{Client, Clip, ClipInner, Layer, Project, Ptr, Stroke};

use crate::{Presence, PresenceData, SelectTool, ToolDyn, Window, WindowInstance};

use crate::{Selection, SelectionKind};

use super::{ProjectState, ScenePreview};

pub struct EditorState {
    pub time: f32,
    pub playing: bool,

    pub open_clip: Ptr<Clip>,
    pub active_layer: Ptr<Layer>,

    pub curr_tool: Rc<RefCell<Box<dyn ToolDyn>>>,

    pub selection: Selection,

    pub will_undo: bool,
    pub will_redo: bool,

    pub hidden_layers: HashSet<Ptr<Layer>>,
    pub locked_layers: HashSet<Ptr<Layer>>,

    pub show_onion_skin: bool,
    pub onion_skin_prev_frames: u32,
    pub onion_skin_next_frames: u32,

    pub stroke_mesh_cache: RefCell<HashMap<Ptr<Stroke>, malvina::StrokeMesh>>,

    pub preview: ScenePreview,

    pub color: pierro::Color,

    windows_to_open: Vec<WindowInstance>,

    on_load_callbacks: Vec<Box<dyn Fn(&ProjectState, &mut EditorState) -> bool>>,

    pub presence: Presence,
    pub other_clients: HashMap<u64, PresenceData>,

    #[cfg(debug_assertions)]
    pub send_messages: bool,
    #[cfg(debug_assertions)]
    pub receive_messages: bool
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

            will_undo: false,
            will_redo: false,

            hidden_layers: HashSet::new(),
            locked_layers: HashSet::new(),

            show_onion_skin: false,
            onion_skin_prev_frames: 2,
            onion_skin_next_frames: 2,

            stroke_mesh_cache: RefCell::new(HashMap::new()),

            preview: ScenePreview::new(),

            color: pierro::Color::BLACK,

            windows_to_open: Vec::new(),

            on_load_callbacks: Vec::new(),

            presence: Presence::new(),
            other_clients: HashMap::new(), 

            #[cfg(debug_assertions)]
            send_messages: true,
            #[cfg(debug_assertions)]
            receive_messages: true,
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

    pub fn jump_to_prev_frame(&mut self, client: &Client, clip: &ClipInner) {
        let Some(layer) = client.get(self.active_layer) else { return; };
        let time = clip.frame_idx(self.time); 
        let Some(frame_ptr) = layer.frame_before(client, time) else { return; };
        let Some(frame) = client.get(frame_ptr) else { return; };
        self.jump_to(clip.frame_len() * (frame.time as f32));
    }

    pub fn jump_to_next_frame(&mut self, client: &Client, clip: &ClipInner) {
        let Some(layer) = client.get(self.active_layer) else { return; };
        let time = clip.frame_idx(self.time); 
        let Some(frame_ptr) = layer.frame_after(client, time) else { return; };
        let Some(frame) = client.get(frame_ptr) else { return; };
        self.jump_to(clip.frame_len() * (frame.time as f32));
    }

    pub fn open_clip(&mut self, clip_ptr: Ptr<Clip>) {
        self.open_clip = clip_ptr;
        self.jump_to(0.0);
        self.active_layer = Ptr::null();
        self.presence.set_open_clip(clip_ptr);
    }

    pub fn open_window<W: Window + 'static>(&mut self, window: W) {
        self.windows_to_open.push(WindowInstance::new(window));
    }

    pub fn open_queued_windows(&mut self, windows: &mut pierro::WindowManager<WindowInstance>) {
        'windows: for window in std::mem::replace(&mut self.windows_to_open, Vec::new()) {
            if window.unique() {
                for existing_window in windows.iter() {
                    if existing_window.type_id() == window.type_id() {
                        continue 'windows;
                    }
                }
            }
            windows.open_window(window);
        }
    }

    pub fn on_load<O: project::alisa::Object<Project = Project>, F: Fn(&ProjectState, &mut EditorState, &O) + 'static>(&mut self, project: &ProjectState, obj_ptr: Ptr<O>, on_load: F) {
        if let Some(obj) = project.client.get(obj_ptr) {
            on_load(project, self, obj);
            return;
        }

        project.client.request_load(obj_ptr);
        self.on_load_callbacks.push(Box::new(move |project, editor| {
            if let Some(obj) = project.client.get(obj_ptr) {
                on_load(project, editor, obj);
                true
            } else {
                false
            }
        }));
    }
    
    pub fn process_on_load_callbacks(&mut self, project: &ProjectState) {
        let mut callbacks = std::mem::replace(&mut self.on_load_callbacks, Vec::new());
        callbacks.retain(|callback| !callback(project, self));
        self.on_load_callbacks.append(&mut callbacks);
    }

    pub fn stroke_transform(&self, stroke: Ptr<Stroke>) -> elic::Mat4 {
        if self.selection.selected(stroke) {
            self.preview.selection_transform
        } else {
            elic::Mat4::IDENTITY
        }
    }

    pub fn can_modify_layer(&self, layer: Ptr<Layer>) -> bool {
        !self.locked_layers.contains(&layer) && !self.hidden_layers.contains(&layer)
    }

}
