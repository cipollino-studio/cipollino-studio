
use super::{Panel, PanelContext};

#[derive(Default)]
pub struct ToolSettings {

}

impl Panel for ToolSettings {
    const NAME: &'static str = "Tool Settings";

    fn title(&self) -> String {
        "Tool Settings".into()
    }

    fn render(&mut self, ui: &mut pierro::UI, context: &mut PanelContext) {
        let curr_tool = context.editor.curr_tool.clone();
        curr_tool.borrow_mut().settings(ui, context.project, context.editor, context.systems, context.renderer);
    }
}
