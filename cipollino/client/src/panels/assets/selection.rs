
use std::collections::HashSet;

use project::{Action, Client, Clip, Folder, Ptr, TransferClip, TransferFolder};

use crate::ProjectState;

use super::AssetUI;

#[derive(Clone, Default)]
pub struct AssetSelection {
    pub folders: HashSet<Ptr<Folder>>,
    pub clips: HashSet<Ptr<Clip>>
}

impl AssetSelection {

    pub fn transfer(self, new_parent: Ptr<Folder>, state: &ProjectState) {
        let mut action = Action::new(); 
        for moved_folder in self.folders {
            state.client.perform(&mut action, TransferFolder {
                ptr: moved_folder,
                new_parent: new_parent 
            });
        }
        for moved_clip in self.clips {
            state.client.perform(&mut action, TransferClip {
                ptr: moved_clip,
                new_folder: new_parent,
            });
        }
        state.undo_redo.add(action);
    }

    pub fn select<A: AssetUI>(&mut self, asset: Ptr<A>) {
        A::selection_list_mut(self).insert(asset);
    }

    pub fn single<A: AssetUI>(asset: Ptr<A>) -> Self {
        let mut selection = Self::default();
        selection.select(asset);
        selection
    }

    fn render_contents_of_asset<A: AssetUI>(&self, ui: &mut pierro::UI, client: &Client) {
        for asset_ptr in A::selection_list(self).iter() {
            let Some(asset) = client.get(*asset_ptr) else { continue; };
            pierro::horizontal_fit_centered(ui, |ui| {
                pierro::icon(ui, A::ICON);
                pierro::label(ui, asset.name());
            });
        }
    }

    pub fn render_contents(&self, ui: &mut pierro::UI, client: &Client) {
        self.render_contents_of_asset::<Folder>(ui, client);
        self.render_contents_of_asset::<Clip>(ui, client);
    }

}
