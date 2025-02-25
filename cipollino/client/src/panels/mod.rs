
mod assets;
pub use assets::*;

mod timeline;
pub use timeline::*;

mod scene;
pub use scene::*;

mod panel;
pub use panel::*;

pub const PANEL_KINDS: &'static [PanelKind] = &[
    PanelKind::of::<AssetsPanel>(),
    PanelKind::of::<TimelinePanel>(),
    PanelKind::of::<ScenePanel>()
];

use crate::UserPref;

pub struct DockingLayoutPref;

impl UserPref for DockingLayoutPref {
    type Type = pierro::DockingState<EditorPanel>;

    fn default() -> Self::Type {
        pierro::DockingState::new(vec![
            EditorPanel::new::<ScenePanel>(),
            EditorPanel::new::<AssetsPanel>(),
        ])
    }

    fn name() -> &'static str {
        "docking_layout"
    }

}
