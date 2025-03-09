
use std::collections::HashSet;

use project::{alisa::Action, DeleteFrame, Frame, Ptr, SetFrameTime};

use crate::ProjectState;

use super::FrameArea;

pub struct FrameSelection {
    frames: HashSet<Ptr<Frame>>
}

impl FrameSelection {

    pub fn new() -> Self {
        Self {
            frames: HashSet::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    pub fn select_frame(&mut self, frame: Ptr<Frame>) {
        self.frames.insert(frame);
    }

    pub fn invert_select_frame(&mut self, frame: Ptr<Frame>) {
        if !self.frames.remove(&frame) {
            self.frames.insert(frame);
        }
    }

    pub fn is_frame_selected(&mut self, frame: Ptr<Frame>) -> bool {
        self.frames.contains(&frame)
    }

    pub fn clear(&mut self) {
        self.frames.clear();
    }
    
    pub fn move_selected(&self, project: &ProjectState, drag: f32) {
        let mut action = Action::new();

        let frame_offset = FrameArea::drag_to_frame_offset(drag);

        let mut selected_frames = Vec::new();
        for frame_ptr in &self.frames {
            if let Some(frame) = project.client.get(*frame_ptr) {
                selected_frames.push((frame.time, *frame_ptr));
            }
        }
        selected_frames.sort_by_key(|(time, _)| *time);
        if frame_offset > 0 {
            selected_frames.reverse();
        }
        for (time, frame) in selected_frames {
            action.push(SetFrameTime {
                frame,
                new_time: time + frame_offset,
                ..Default::default()
            });
        } 
         
        project.client.queue_action(action);
    }

    pub fn delete(&self, project: &ProjectState) {
        let mut action = Action::new();
        for frame in self.frames.iter() {
            action.push(DeleteFrame {
                ptr: *frame
            });
        }
        project.client.queue_action(action);
    }

}
