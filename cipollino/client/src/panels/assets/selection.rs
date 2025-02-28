
use std::collections::HashSet;

use project::{alisa::{self, Action, TreeObj}, deep_load_clip, deep_load_folder, Client, Clip, DeleteClip, DeleteFolder, Folder, Ptr, TransferClip, TransferFolder, UndoRedoManager};

use crate::ProjectState;

use super::AssetUI;

#[derive(Clone, Default)]
pub struct AssetSelection {
    pub folders: HashSet<Ptr<Folder>>,
    pub clips: HashSet<Ptr<Clip>>
}

impl AssetSelection {

    pub fn deep_load_all(&self, client: &Client) {
        for folder in self.folders.iter() {
            deep_load_folder(*folder, client);
        }
        for clip in self.clips.iter() {
            deep_load_clip(*clip, client);
        }
    }

    pub fn try_delete(&self, client: &Client, undo_redo: &UndoRedoManager) -> bool {
        // Check to make sure we can delete the selection
        for folder in self.folders.iter() {
            if !Folder::can_delete(*folder, &client.context(), alisa::OperationSource::Local) {
                return false;
            }
        }
        for clip in self.clips.iter() {
            if !Clip::can_delete(*clip, &client.context(), alisa::OperationSource::Local) {
                return false;
            }
        }

        let mut action = Action::new();
        for folder in self.folders.iter() {
            client.perform(&mut action, DeleteFolder {
                ptr: *folder,
            });
        }
        for clip in self.clips.iter() {
            client.perform(&mut action, DeleteClip {
                ptr: *clip,
            });
        }

        undo_redo.add(action);

        true
    }

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
