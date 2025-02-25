
use project::{Action, Clip, CreateLayer, LayerParent, LayerTreeData, Ptr};

use crate::ProjectState;

use super::TimelinePanel;

impl TimelinePanel {

    pub(super) fn header(&mut self, ui: &mut pierro::UI, clip_ptr: Ptr<Clip>, clip: &Clip, project: &ProjectState) {
        if pierro::icon_button(ui, pierro::icons::FILE_PLUS).mouse_clicked() {
            if let Some(ptr) = project.client.next_ptr() {
                let mut action = Action::new();
                project.client.perform(&mut action, CreateLayer {
                    ptr,
                    parent: LayerParent::Clip(clip_ptr),
                    idx: clip.layers.n_children(),
                    data: LayerTreeData {
                        name: "Layer".to_owned(),
                    },
                });
                project.undo_redo.add(action);
            }
        }

        pierro::h_spacing(ui, 5.0);
        pierro::v_line(ui);
        pierro::h_spacing(ui, 5.0);

        pierro::label(ui, clip.length.to_string());
    }

}