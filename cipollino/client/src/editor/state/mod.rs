

mod project_state;
pub use project_state::*;

mod editor_state;
pub use editor_state::*;

mod preview;
pub use preview::*;

pub struct State {
    pub project: ProjectState,
    pub editor: EditorState,
    pub renderer: Option<malvina::Renderer>
}
