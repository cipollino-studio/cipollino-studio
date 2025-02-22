
use crate::State;

use super::Panel;

#[derive(Default)]
pub struct ScenePanel {

}

impl Panel for ScenePanel {

    const NAME: &'static str = "Scene";

    fn title(&self) -> String {
        "Scene".to_owned()
    }

    fn render(&mut self, ui: &mut pierro::UI, state: &mut State) {
        let Some(clip) = state.project.client.get(state.editor.open_clip) else {
            pierro::centered(ui, |ui| {
                pierro::label(ui, "No clip open.");
            });
            return;
        };
        pierro::label(ui, &clip.name);
    }

}
