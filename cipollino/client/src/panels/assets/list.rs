
use std::collections::HashSet;

use project::{alisa::{self, TreeObj}, deep_load_audio_clip, deep_load_clip, deep_load_folder, deep_load_palette, Action, AudioClip, Client, Clip, DeleteAudioClip, DeleteClip, DeleteFolder, DeletePalette, Folder, Palette, Ptr, TransferAudioClip, TransferClip, TransferFolder, TransferPalette};

use crate::{EditorState, ProjectState};

use super::AssetUI;
 
#[derive(Default, Clone)]
pub struct AssetList {
    pub folders: HashSet<Ptr<Folder>>,
    pub clips: HashSet<Ptr<Clip>>,
    pub palettes: HashSet<Ptr<Palette>>,
    pub audio_clips: HashSet<Ptr<AudioClip>>
}

impl AssetList {

    pub fn deep_load_all(&self, client: &Client) {
        for folder in self.folders.iter() {
            deep_load_folder(*folder, client);
        }
        for clip in self.clips.iter() {
            deep_load_clip(*clip, client);
        }
        for palette in self.palettes.iter() {
            deep_load_palette(*palette, client);
        }
        for audio_clip in self.audio_clips.iter() {
            deep_load_audio_clip(*audio_clip, client);
        }
    }

    pub fn try_delete(&self, client: &Client, editor: &EditorState) -> bool {
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
        for palette in self.palettes.iter() {
            if !Palette::can_delete(*palette, &client.context(), alisa::OperationSource::Local) {
                return false;
            }
        }
        for audio_clip in self.audio_clips.iter() {
            if !AudioClip::can_delete(*audio_clip, &client.context(), alisa::OperationSource::Local) {
                return false;
            }
        }

        let mut action = Action::new(editor.action_context("Delete Assets"));
        for folder in self.folders.iter() {
            action.push(DeleteFolder {
                ptr: *folder,
            });
        }
        for clip in self.clips.iter() {
            action.push(DeleteClip {
                ptr: *clip,
            });
        }
        for palette in self.palettes.iter() {
            action.push(DeletePalette {
                ptr: *palette,
            });
        }
        for audio_clip in self.audio_clips.iter() {
            action.push(DeleteAudioClip {
                ptr: *audio_clip,
            });
        }

        client.queue_action(action);

        true
    }

    pub fn transfer(self, new_parent: Ptr<Folder>, project: &ProjectState, editor: &EditorState) {
        let mut action = Action::new(editor.action_context("Transfer Assets")); 
        for moved_folder in self.folders {
            action.push(TransferFolder {
                ptr: moved_folder,
                new_parent: new_parent 
            });
        }
        for moved_clip in self.clips {
            action.push(TransferClip {
                ptr: moved_clip,
                new_folder: new_parent,
            });
        }
        for moved_palette in self.palettes {
            action.push(TransferPalette {
                ptr: moved_palette,
                new_folder: new_parent,
            });
        }
        for moved_audio_clip in self.audio_clips {
            action.push(TransferAudioClip {
                ptr: moved_audio_clip,
                new_folder: new_parent,
            });
        }
        project.client.queue_action(action);
    }

    pub fn add<A: AssetUI>(&mut self, asset: Ptr<A>) {
        A::asset_list_mut(self).insert(asset);
    }

    pub fn single<A: AssetUI>(asset: Ptr<A>) -> Self {
        let mut selection = Self::default();
        selection.add(asset);
        selection
    }

    fn render_contents_of_asset<A: AssetUI>(&self, ui: &mut pierro::UI, client: &Client) {
        for asset_ptr in A::asset_list(self).iter() {
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
        self.render_contents_of_asset::<Palette>(ui, client);
        self.render_contents_of_asset::<AudioClip>(ui, client);
    }

}