
mod assets;
pub use assets::*;

mod timeline;
pub use timeline::*;

mod scene;
pub use scene::*;

mod panel;
pub use panel::*;

mod tool_settings;
pub use tool_settings::*;

#[cfg(debug_assertions)]
mod debug;
#[cfg(debug_assertions)]
pub use debug::*;

pub const PANEL_KINDS: &'static [PanelKind] = &[
    PanelKind::of::<AssetsPanel>(),
    PanelKind::of::<TimelinePanel>(),
    PanelKind::of::<ScenePanel>(),
    PanelKind::of::<ToolSettings>(),

    #[cfg(debug_assertions)]
    PanelKind::of::<DebugPanel>()
];

use crate::UserPref;

pub struct DockingLayoutPref;

impl UserPref for DockingLayoutPref {
    type Type = pierro::DockingState<EditorPanel>;

    fn default() -> Self::Type {
        pierro::DockingLayout::Split(pierro::Axis::Y, vec![
            (0.7, pierro::DockingLayout::Split(pierro::Axis::X, vec![
                (0.8, pierro::DockingLayout::Tabs(vec![EditorPanel::new::<ScenePanel>()])),
                (0.2, pierro::DockingLayout::Tabs(vec![EditorPanel::new::<ToolSettings>(), EditorPanel::new::<AssetsPanel>()]))
            ])),
            (0.3, pierro::DockingLayout::Tabs(vec![EditorPanel::new::<TimelinePanel>()]))
        ]).into_state()
    }

    fn name() -> &'static str {
        "docking_layout"
    }

}
