
use project::{Action, CreateStroke, StrokeData, StrokeTreeData};

use super::{Tool, ToolContext};

#[derive(Default)]
pub struct Pencil {

}

impl Tool for Pencil {
    const ICON: &'static str = pierro::icons::PENCIL;

    fn mouse_clicked(&mut self, ctx: &ToolContext, pos: malvina::Vec2) {
        let mut action = Action::new();
        let Some(ptr) = ctx.project.client.next_ptr() else { return; };
        let Some(frame) = ctx.active_frame(&mut action) else { return; };
        let stroke = malvina::Stroke::point(pos, 1.0);
        action.push(CreateStroke {
            ptr,
            parent: frame,
            idx: 0,
            data: StrokeTreeData {
                stroke: StrokeData(stroke),
            },
        });
        ctx.project.client.queue_action(action);
    }

}
