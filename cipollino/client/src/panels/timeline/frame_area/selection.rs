
use std::collections::HashSet;

use project::{alisa::Action, Frame, Ptr, SetFrameTime};

use crate::{ProjectState, TimelinePanel};

pub struct FrameSelection {
    frames: HashSet<Ptr<Frame>>
}

impl FrameSelection {

    pub fn new() -> Self {
        Self {
            frames: HashSet::new(),
        }
    }

    pub fn select_frame(&mut self, frame: Ptr<Frame>) {
        self.frames.insert(frame);
    }

    pub fn is_frame_selected(&mut self, frame: Ptr<Frame>) -> bool {
        self.frames.contains(&frame)
    }

    pub fn clear(&mut self) {
        self.frames.clear();
    }
    
    pub fn move_selected(&self, project: &ProjectState, drag: f32) {
        let mut action = Action::new();

        let frame_offset = TimelinePanel::drag_to_frame_offset(drag);

        for frame_ptr in &self.frames {
            if let Some(frame) = project.client.get(*frame_ptr) {
                project.client.perform(&mut action, SetFrameTime {
                    frame: *frame_ptr,
                    new_time: (frame.time + frame_offset).max(0),
                    ..Default::default()
                });
            }
        }

        project.undo_redo.add(action);
    }

}
