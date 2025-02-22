
mod assets;
pub use assets::*;

mod scene;
pub use scene::*;

mod panel;
pub use panel::*;

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
