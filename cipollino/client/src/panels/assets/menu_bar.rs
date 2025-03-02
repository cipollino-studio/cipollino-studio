
use project::{Action, Clip, Folder, Ptr};

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

    pub(crate) fn menu_bar(&self, ui: &mut pierro::UI, state: &ProjectState) {
        pierro::menu_bar(ui, |ui| {
            self.asset_menu_bar_icon::<Folder>(ui, state); 
            self.asset_menu_bar_icon::<Clip>(ui, state); 
        });   
    }

}
