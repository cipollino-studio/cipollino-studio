
use project::{alisa::Action, Clip, Folder, Ptr};

use crate::{EditorState, ProjectState};

use super::{CreateClipDialog, AssetUI, AssetsPanel};

impl AssetsPanel {

    fn asset_menu_bar_icon<A: AssetUI>(&self, ui: &mut pierro::UI, project: &ProjectState, editor: &EditorState) {
        if pierro::icon_button(ui, A::ICON).mouse_clicked() {
            let mut action = Action::new(editor.action_context(format!("Create {}", A::NAME)));
            A::create(project.client.next_ptr(), Ptr::null(), &mut action); 
            project.client.queue_action(action);
        }
    }
    
    fn clip_menu_bar_icon(&self, ui: &mut pierro::UI, editor: &mut EditorState) {
        if pierro::icon_button(ui, Clip::ICON).mouse_clicked() {
            editor.open_window(CreateClipDialog::new());
        }
    } 

    pub(crate) fn menu_bar(&self, ui: &mut pierro::UI, editor: &mut EditorState, project: &ProjectState) {
        let button_color = ui.style::<pierro::theme::BgDark>();
        ui.with_style::<pierro::theme::BgButton, _, _>(button_color, |ui| {
            ui.with_style::<pierro::theme::WidgetMargin, _, _>(pierro::Margin::same(3.5), |ui| {
                ui.with_style::<pierro::theme::LabelFontSize, _, _>(15.0, |ui| {
                    pierro::menu_bar(ui, |ui| {
                        self.asset_menu_bar_icon::<Folder>(ui, project, editor); 
                        self.clip_menu_bar_icon(ui, editor);
                    });
                });
            });
        });
    }

}
