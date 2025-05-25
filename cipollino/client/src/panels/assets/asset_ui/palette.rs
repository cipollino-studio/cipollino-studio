use std::collections::HashSet;

use alisa::Ptr;
use project::{Action, CreatePalette, Folder, Palette, PaletteTreeData};

use crate::AssetList;

use super::AssetUI;


impl AssetUI for Palette {
    const ICON: &'static str = pierro::icons::PALETTE;

    fn create(ptr: Ptr<Self>, parent: Ptr<Folder>, action: &mut Action) {
        action.push(CreatePalette {
            ptr,
            parent,
            data: PaletteTreeData::default(),
        });
    }

    fn asset_list(list: &AssetList) -> &HashSet<Ptr<Self>> {
        &list.palettes
    }

    fn asset_list_mut(list: &mut AssetList) -> &mut HashSet<Ptr<Self>> {
        &mut list.palettes
    }

}
