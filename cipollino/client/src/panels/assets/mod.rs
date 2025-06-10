
use std::cell::RefCell;
use alisa::Ptr;
use project::alisa::AnyPtr;

use super::{Panel, PanelContext};

mod asset_ui;
use asset_ui::*;

mod tree_ui;
mod menu_bar;

mod list;
pub use list::*;

mod clip_dialog;
pub use clip_dialog::*;

#[derive(Default)]
pub struct AssetsPanel {
    renaming_state: RefCell<Option<(AnyPtr, String)>>,
    started_renaming: RefCell<bool>,
    asset_dnd_source: RefCell<pierro::DndSource>,
}

impl Panel for AssetsPanel {

    const NAME: &'static str = "Assets";

    fn title(&self) -> String {
        "Assets".to_owned()
    }

    fn render(&mut self, ui: &mut pierro::UI, context: &mut PanelContext) {
        self.menu_bar(ui, context.editor, context.project);

        let (_, moved_assets) = pierro::dnd_drop_zone_with_size::<AssetList, _>(ui, pierro::Size::fr(1.0), pierro::Size::fr(1.0), |ui| {
            pierro::scroll_area(ui, |ui| {
                pierro::margin(ui, pierro::Margin::same(3.0), |ui| {
                    self.render_folder_contents(
                        ui,
                        &context.project.client.folders,
                        &context.project.client.clips,
                        &context.project.client.palettes,
                        &context.project.client.audio_clips,
                        &context.project,
                        &mut context.editor
                    ); 
                });
            });
        });
        if let Some(moved_assets) = moved_assets {
            moved_assets.transfer(Ptr::null(), &context.project, &context.editor);
        }

        self.asset_dnd_source.borrow_mut().display(ui, |ui| {
            let Some(assets) = ui.memory().get_dnd_payload::<AssetList>() else {
                ui.memory().clear_dnd_payload();
                return;
            };
            let assets = assets.clone();
            assets.render_contents(ui, &context.project.client); 
        });
    }

}
