
use std::collections::HashSet;

use project::AudioClip;

use crate::AssetList;

use super::AssetUI;

impl AssetUI for AudioClip {
    const ICON: &'static str = pierro::icons::MUSIC_NOTES;

    fn create(_client: &project::Client, _ptr: alisa::Ptr<Self>, _parent: alisa::Ptr<project::Folder>, _action: &mut project::Action) {
        unreachable!()
    }

    fn asset_list(list: &AssetList) -> &HashSet<alisa::Ptr<Self>> {
        &list.audio_clips 
    }

    fn asset_list_mut(list: &mut AssetList) -> &mut HashSet<alisa::Ptr<Self>> {
        &mut list.audio_clips
    }
}
