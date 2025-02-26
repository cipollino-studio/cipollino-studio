
use project::{Client, Clip, Layer, LayerChildList, LayerChildPtr, Ptr};

pub struct EditorState {
    pub time: f32,
    pub playing: bool,

    pub open_clip: Ptr<Clip>,
    pub active_layer: Ptr<Layer>
}

impl EditorState {

    pub fn jump_to(&mut self, time: f32) {
        self.time = time;
        self.playing = false;
    }

    pub(super) fn tick_playback(&mut self, ui: &mut pierro::UI, clip: &Clip) {
        if self.playing {
            self.time += ui.input().delta_time;
            ui.request_redraw();
        }

        if self.time > clip.duration() {
            self.time = 0.0;
        }
        self.time = self.time.max(0.0);
    }

    fn find_first_layer(_client: &Client, layers: &LayerChildList) -> Ptr<Layer> {
        for child in layers.iter() {
            match child {
                LayerChildPtr::Layer(layer) => {
                    return layer.ptr();
                },
            }
        } 

        Ptr::null()
    }

    pub fn open_clip(&mut self, client: &Client, clip_ptr: Ptr<Clip>) {
        let Some(clip) = client.get(clip_ptr) else { return; };
        self.open_clip = clip_ptr;
        self.jump_to(0.0);
        self.active_layer = Self::find_first_layer(client, &clip.layers);
    }

}
