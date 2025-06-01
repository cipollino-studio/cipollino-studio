
mod project_state;
pub use project_state::*;

mod editor_state;
pub use editor_state::*;

mod preview;
pub use preview::*;

mod renderer_state;
pub use renderer_state::*;

pub struct State {
    pub project: ProjectState,
    pub editor: EditorState,
    pub renderer: Option<RendererState>
}
