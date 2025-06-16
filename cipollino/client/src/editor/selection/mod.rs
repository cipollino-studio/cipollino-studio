use std::collections::HashSet;

use project::{alisa::Object, AudioInstance, Clip, Fill, Folder, Frame, Layer, Ptr, Stroke};

mod selectable;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SelectionKind {
    None,
    Asset,
    Layers,
    Frames,
    Scene
}

pub trait Selectable: Object {

    const KIND: SelectionKind;

    fn selection_list(selection: &Selection) -> &HashSet<Ptr<Self>>;
    fn selection_list_mut(selection: &mut Selection) -> &mut HashSet<Ptr<Self>>;

}

pub struct Selection {
    kind: SelectionKind,
    folders: HashSet<Ptr<Folder>>,
    clips: HashSet<Ptr<Clip>>,
    layers: HashSet<Ptr<Layer>>,
    frames: HashSet<Ptr<Frame>>,
    audio_instances: HashSet<Ptr<AudioInstance>>,
    strokes: HashSet<Ptr<Stroke>>,
    fills: HashSet<Ptr<Fill>>,

    shift_down: bool,
    keep_selection: bool,

    /// A counter that gets incremented every time the selection is modified.
    /// Used for efficiently checking if some selection-based cache is invalid(e.g. the select tool's free transform bounding box)
    version: u64
}

impl Selection {

    pub fn new() -> Self {
        Self {
            kind: SelectionKind::None,
            folders: HashSet::new(),
            clips: HashSet::new(),
            layers: HashSet::new(),
            frames: HashSet::new(),
            audio_instances: HashSet::new(),
            strokes: HashSet::new(),
            fills: HashSet::new(),
            shift_down: false,
            keep_selection: false,
            version: 0,
        }
    }

    pub fn begin_frame(&mut self, shift_down: bool) {
        self.shift_down = shift_down;
        self.keep_selection = false;
    }

    pub fn end_frame(&mut self, clicked: bool) {
        if !self.keep_selection && clicked && !self.shift_down {
            self.clear();
        }
    }

    pub fn clear(&mut self) {
        self.kind = SelectionKind::None;
        self.folders.clear();
        self.clips.clear();
        self.layers.clear();
        self.frames.clear();
        self.audio_instances.clear();
        self.strokes.clear();
        self.fills.clear();
        self.version += 1;
    }

    pub fn selected<S: Selectable>(&self, ptr: Ptr<S>) -> bool {
        S::selection_list(self).contains(&ptr)
    }

    pub fn select<S: Selectable>(&mut self, ptr: Ptr<S>) {
        if ptr.is_null() {
            return;
        }
        if self.kind != S::KIND {
            self.clear();
        }
        S::selection_list_mut(self).insert(ptr);
        self.kind = S::KIND;
        self.keep_selection = true;
        self.version += 1;
    }

    pub fn invert_select<S: Selectable>(&mut self, ptr: Ptr<S>) {
        if !S::selection_list_mut(self).remove(&ptr) {
            self.select(ptr);
        } else {
            self.version += 1;
        }
        self.keep_selection = true;
    }

    pub fn extend_select<S: Selectable>(&mut self, ptr: Ptr<S>) {
        if !self.shift_down {
            self.clear();
        }
        self.invert_select(ptr);
    } 

    pub fn keep_selection(&mut self) {
        self.keep_selection = true;
    }

    pub fn iter<S: Selectable>(&self) -> impl Iterator<Item = Ptr<S>> + '_ {
        S::selection_list(self).iter().cloned()
    }
    
    pub fn retain<S: Selectable, F: Fn(Ptr<S>) -> bool>(&mut self, filter: F) {
        let initial_size = S::selection_list(self).len();
        S::selection_list_mut(self).retain(|ptr| filter(*ptr));
        if S::selection_list(self).len() < initial_size {
            self.version += 1;
        }
    }

    pub fn kind(&self) -> SelectionKind {
        self.kind
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn shift_down(&self) -> bool {
        self.shift_down
    }

    pub fn replace(&mut self, mut new_selection: Self) {
        new_selection.version = self.version + 1; 
        *self = new_selection;
    }

    pub fn contains<S: Selectable>(&self) -> bool {
        !S::selection_list(self).is_empty()
    }

}
