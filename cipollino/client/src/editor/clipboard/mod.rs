
mod scene;
use project::Client;
pub use scene::*;

mod timeline;
pub use timeline::*;

use super::{EditorState, LayerRenderList, SceneRenderList, Selection, SelectionKind};

pub enum Clipboard {
    Timeline(TimelineClipboard),
    Scene(SceneClipboard)
}

impl Selection {

    pub fn collect_clipboard(&self, client: &Client, editor: &EditorState, layer_render_list: Option<&LayerRenderList>, scene_render_list: Option<&SceneRenderList>) -> Option<Clipboard> {
        match self.kind() {
            SelectionKind::None => None,
            SelectionKind::Asset => None,
            SelectionKind::Layers => None,
            SelectionKind::Frames => Some(Clipboard::Timeline(self.collect_timeline_clipboard(client, editor, layer_render_list?))),
            SelectionKind::Scene => Some(Clipboard::Scene(self.collect_scene_clipboard(client, scene_render_list?))),
        }
    } 

}

impl Clipboard {

    pub fn paste(&self, client: &Client, editor: &EditorState, layer_render_list: Option<&LayerRenderList>) -> Option<Selection> {
        match self {
            Clipboard::Timeline(timeline) => {
                timeline.paste(client, editor, layer_render_list?)
            },
            Clipboard::Scene(scene) => {
                scene.paste(client, editor)
            },
        }
    }

}
