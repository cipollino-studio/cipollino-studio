
use project::{alisa::Action, ActionContext, Clip, Folder, Ptr};

use crate::{ProjectState, State};

use super::{CreateClipDialog, AssetUI, AssetsPanel};

impl AssetsPanel {

    fn asset_menu_bar_icon<A: AssetUI>(&self, ui: &mut pierro::UI, state: &ProjectState) {
        if pierro::icon_button(ui, A::ICON).mouse_clicked() {
            if let Some(ptr) = state.client.next_ptr() {
                let mut action = Action::new(ActionContext::new(format!("Create {}", A::NAME)));
                A::create(ptr, Ptr::null(), &mut action); 
                state.client.queue_action(action);
            }
        }
    }

    fn create_clip(state: &mut State) {
        state.editor.open_window(CreateClipDialog::new());
    }
    
    fn clip_menu_bar_icon(&self, ui: &mut pierro::UI, state: &mut State) {
        if pierro::icon_button(ui, Clip::ICON).mouse_clicked() {
            Self::create_clip(state);
        }
    } 

    pub(crate) fn menu_bar(&self, ui: &mut pierro::UI, state: &mut State) {
        let button_color = ui.style::<pierro::theme::BgDark>();
        ui.with_style::<pierro::theme::BgButton, _, _>(button_color, |ui| {
            pierro::menu_bar(ui, |ui| {
                self.asset_menu_bar_icon::<Folder>(ui, &state.project); 
                self.clip_menu_bar_icon(ui, state);
            });
        });
    }

}
