
use project::{alisa::Action, Clip, ClipTreeData, CreateClip, Folder, Ptr};

use crate::ProjectState;

use super::{AssetUI, AssetsPanel};

impl AssetsPanel {

    fn asset_menu_bar_icon<A: AssetUI>(&self, ui: &mut pierro::UI, state: &ProjectState) {
        if pierro::icon_button(ui, A::ICON).mouse_clicked() {
            if let Some(ptr) = state.client.next_ptr() {
                let mut action = Action::new();
                A::create(ptr, Ptr::null(), &mut action); 
                state.client.queue_action(action);
            }
        }
    }

    fn create_clip(state: &ProjectState) {
        let Some(clip_ptr) = state.client.next_ptr() else { return; };
        let Some(inner_ptr) = state.client.next_ptr() else { return; };
        state.client.queue_action(Action::single(CreateClip {
            ptr: clip_ptr,
            parent: Ptr::null(),
            data: ClipTreeData {
                name: "Clip".to_owned(),
                length: 100,
                framerate: 24.0,
                inner_ptr,
                ..Default::default()
            },
        }));
    }
    
    fn clip_menu_bar_icon(&self, ui: &mut pierro::UI, state: &ProjectState) {
        if pierro::icon_button(ui, Clip::ICON).mouse_clicked() {
            Self::create_clip(state);
        }
    } 

    pub(crate) fn menu_bar(&self, ui: &mut pierro::UI, state: &ProjectState) {
        pierro::menu_bar(ui, |ui| {
            self.asset_menu_bar_icon::<Folder>(ui, state); 
            self.clip_menu_bar_icon(ui, state);
        });
    }

}
