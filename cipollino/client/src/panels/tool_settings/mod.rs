
use crate::State;

use super::Panel;

#[derive(Default)]
pub struct ToolSettings {

}

impl Panel for ToolSettings {
    const NAME: &'static str = "Tool Settings";

    fn title(&self) -> String {
        "Tool Settings".into()
    }

    fn render(&mut self, ui: &mut pierro::UI, state: &mut State) {
        state.editor.curr_tool.borrow_mut().settings(ui);
    }
}
