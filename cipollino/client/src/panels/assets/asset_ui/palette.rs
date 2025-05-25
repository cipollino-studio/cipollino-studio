use std::collections::HashSet;

use alisa::Ptr;
use project::{Action, AddPaletteToClip, CreatePalette, CreatePaletteInner, Folder, Palette, PaletteTreeData};

use crate::{AssetList, EditorState, ProjectState};

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

    fn on_open(ptr: Ptr<Self>, project: &ProjectState, editor: &mut EditorState) {
        let Some(clip) = project.client.get(editor.open_clip) else { return; }; 
        if project.client.get(clip.inner).is_none() {
            return;
        }

        let Some(palette) = project.client.get(ptr) else { return; };
        let palette_inner = if palette.inner.is_null() {
            let new_inner_ptr = project.client.next_ptr();
            project.client.queue_operation(CreatePaletteInner {
                palette: ptr,
                inner: new_inner_ptr,
            });
            new_inner_ptr
        } else {
            project.client.request_load(palette.inner);
            palette.inner
        };
        
        project.client.queue_action(Action::single(editor.action_context("Add Palette to Clip"), AddPaletteToClip {
            clip: clip.inner,
            palette: palette_inner,
        }));
    }

}
