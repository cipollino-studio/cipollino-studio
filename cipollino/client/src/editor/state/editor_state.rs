
use std::collections::HashSet;
use std::{cell::RefCell, collections::HashMap};
use std::rc::Rc;
use alisa::Object;
use project::{AudioLayer, Client, ClientId, Clip, ClipInner, Layer, LayerGroup, PresenceData, Project, Ptr, SceneObjPtr, SceneObjectColor, StrokeBrush};

use crate::{AppSystems, AudioBlockCache, Clipboard, MeshCache, Presence, SelectTool, Selectable, ToolDyn, Window, WindowInstance};

use crate::{Selection, SelectionKind};

use super::{ProjectState, ScenePreview};

pub struct EditorState {
    pub time: f32,
    pub playing: bool,
    pub jumped: bool,

    pub open_clip: Ptr<Clip>,
    pub active_layer: Ptr<Layer>,

    pub curr_tool: Rc<RefCell<Box<dyn ToolDyn>>>,

    pub selection: Selection,
    pub next_selection: Option<Selection>, 
    pub clipboard: Option<Clipboard>,

    pub will_undo: bool,
    pub will_redo: bool,

    pub hidden_layers: HashSet<Ptr<Layer>>,
    pub locked_layers: HashSet<Ptr<Layer>>,
    pub muted_layers: HashSet<Ptr<AudioLayer>>,
    pub open_layer_groups: HashSet<Ptr<LayerGroup>>,

    pub show_onion_skin: bool,
    pub onion_skin_prev_frames: u32,
    pub onion_skin_next_frames: u32,

    pub mesh_cache: MeshCache,
    pub audio_cache: AudioBlockCache,

    pub preview: ScenePreview,

    pub color: SceneObjectColor,
    pub brush: StrokeBrush,

    windows_to_open: Vec<WindowInstance>,

    on_load_callbacks: Vec<Box<dyn Fn(&ProjectState, &mut EditorState) -> bool>>,

    pub presence: Presence,
    pub other_clients: HashMap<ClientId, PresenceData>,

    #[cfg(debug_assertions)]
    pub send_messages: bool,
    #[cfg(debug_assertions)]
    pub receive_messages: bool
}

impl EditorState {

    pub fn new(systems: &mut AppSystems) -> Self {
        Self {
            time: 0.0,
            playing: false,
            jumped: false,

            open_clip: Ptr::null(),
            active_layer: Ptr::null(),
            
            curr_tool: Rc::new(RefCell::new(Box::new(SelectTool::default()))),

            selection: Selection::new(),
            next_selection: None,
            clipboard: None,

            will_undo: false,
            will_redo: false,

            hidden_layers: HashSet::new(),
            locked_layers: HashSet::new(),
            muted_layers: HashSet::new(),
            open_layer_groups: HashSet::new(),

            show_onion_skin: false,
            onion_skin_prev_frames: 2,
            onion_skin_next_frames: 2,

            mesh_cache: MeshCache::new(),
            audio_cache: AudioBlockCache::new(systems.audio.sample_rate()),

            preview: ScenePreview::new(),

            color: SceneObjectColor::default(),
            brush: StrokeBrush::default(),

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
        self.jumped = true;
    }

    pub fn tick_playback(&mut self, ui: &mut pierro::UI, systems: &mut AppSystems, clip: &ClipInner) {

        if self.jumped {
            self.jumped = false;
            systems.audio.set_time((self.time * (systems.audio.sample_rate() as f32)).round() as i64);
        }

        self.time = (systems.audio.time() as f32) / (systems.audio.sample_rate() as f32);

        if self.playing {
            ui.request_redraw();
            if self.selection.kind() == SelectionKind::Scene {
                self.selection.clear();
            }
            systems.audio.play();
        } else {
            systems.audio.pause();
        }

        if self.time > clip.duration() || self.time < 0.0 {
            self.time = 0.0;
            self.jumped = true;
        }
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

    pub fn scene_obj_transform<O: Object>(&self, stroke: Ptr<O>) -> elic::Mat4 where O: Selectable, SceneObjPtr: From<Ptr<O>> {
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
