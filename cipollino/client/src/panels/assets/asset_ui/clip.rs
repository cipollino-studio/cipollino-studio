use std::collections::HashSet;

use alisa::Ptr;
use project::{Action, Clip, ClipTreeData, CreateClip, Folder};

use crate::{presence_color, presence_icon, AssetList, ClipProperties, ClipPropertiesDialog, EditorState, ProjectState};

use super::AssetUI;


impl AssetUI for Clip {
    const ICON: &'static str = pierro::icons::FILM_STRIP;

    fn create(ptr: Ptr<Self>, parent: Ptr<Folder>, action: &mut Action) {
        action.push(CreateClip {
            ptr,
            parent,
            data: ClipTreeData::default(),
        });
    }

    fn on_open(clip: Ptr<Self>, _project: &ProjectState, state: &mut EditorState) {
        state.open_clip(clip);
    }

    fn asset_list(list: &AssetList) -> &HashSet<Ptr<Self>> {
        &list.clips
    }

    fn asset_list_mut(list: &mut AssetList) -> &mut HashSet<Ptr<Self>> {
        &mut list.clips
    }

    fn context_menu(ui: &mut pierro::UI, project: &ProjectState, editor: &mut EditorState, clip_ptr: Ptr<Self>, context_menu_id: pierro::Id) {
        if pierro::menu_button(ui, "Properties...").mouse_clicked() {
            if let Some(clip) = project.client.get(clip_ptr) {
                let name = clip.name.clone();
                let clip_inner_ptr = clip.inner;
                editor.on_load(project, clip_inner_ptr, move |_, editor, clip_inner| {
                    let properties = ClipProperties {
                        name: name.clone(),
                        length: clip_inner.length,
                        framerate: clip_inner.framerate,
                        width: clip_inner.width,
                        height: clip_inner.height,
                        background_color: clip_inner.background_color,
                    };

                    editor.open_window(ClipPropertiesDialog {
                        properties,
                        clip_ptr,
                        clip_inner_ptr
                    });
                });
            }
            pierro::close_context_menu(ui, context_menu_id);
        }
    }

    fn label_ui(ui: &mut pierro::UI, ptr: Ptr<Self>, state: &mut EditorState) {
        pierro::h_spacing(ui, 5.0);
        for (client_id, presence) in state.other_clients.iter() {
            if presence.open_clip == ptr {
                let color = presence_color(*client_id);
                let icon = presence_icon(*client_id);
                ui.with_node(
                    pierro::UINodeParams::new(pierro::Size::fit(), pierro::Size::fit())
                        .with_fill(color)
                        .with_layout(pierro::Layout::vertical().align_center().justify_center())
                        .with_rounding(pierro::Rounding::same(3.0))
                        .with_margin(pierro::Margin::same(1.0)),
                    |ui| {
                        let text_color = ui.style::<pierro::theme::ActiveTextColor>();
                        ui.with_style::<pierro::theme::TextColor, _, _>(text_color, |ui| {
                            pierro::icon(ui, icon)
                        });
                    }
                );
                pierro::h_spacing(ui, 2.0);
            }
        }
    }

}
