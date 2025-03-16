
use crate::{frame::find_frame_at_time, Layer, Project};

use super::{Frame, FrameTreeData};

alisa::object_set_property_delta!(Frame, time, i32);

#[derive(alisa::Serializable, Default)]
#[project(Project)]
pub struct SetFrameTime {
    pub frame: alisa::Ptr<Frame>,
    pub new_time: i32,
    
    pub frame_recreation_ptr: alisa::Ptr<Frame>,
    pub frame_recreation_data: Option<FrameTreeData>
}

impl alisa::Operation for SetFrameTime {
    type Project = Project;
    type Inverse = SetFrameTime;

    const NAME: &'static str = "SetFrameTime";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Project>) -> bool {

        use alisa::TreeObj;

        let new_time = self.new_time.max(0);

        let Some(frame) = recorder.obj_list().get(self.frame) else { return false; };
        let layer: alisa::Ptr<Layer> = frame.layer;

        // If there's already a frame at the time we're moving this frame to, delete it
        let context = recorder.context();
        let Some(child_list) = Frame::child_list(layer, &context) else { return false; };
        if let Some(other_frame) = find_frame_at_time(&context, child_list, new_time) {
            if other_frame != self.frame {
                alisa::delete_tree_object(recorder, other_frame);
            }
        }

        // Update the frame's time
        let Some(frame) = recorder.obj_list_mut().get_mut(self.frame) else { return false; };
        let old_time = frame.time;
        frame.time = new_time;
        recorder.push_delta(SetFrameTimeDelta {
            ptr: self.frame,
            time_value: old_time,
        });

        // Recreated a previously deleted frame if necessary 
        if let Some(data) = &self.frame_recreation_data {
            alisa::create_tree_object(recorder, self.frame_recreation_ptr, layer, (), data);
        }

        true
    }

    fn inverse(&self, context: &alisa::ProjectContext<Project>) -> Option<SetFrameTime> {

        use alisa::TreeObj;

        let frame = context.obj_list().get(self.frame)?;
        let old_time = frame.time;
        let layer = frame.layer;
        let new_time = self.new_time.max(0);

        // If we're going to delete a frame when we move this frame, we need to collect its tree data to recreate it
        let child_list = Frame::child_list(layer, context)?;
        let recreated_frame = find_frame_at_time(&context, child_list, new_time).unwrap_or_default();
        let recreated_frame = if recreated_frame == self.frame {
            alisa::Ptr::null()
        } else {
            recreated_frame
        };
        let recreate_data = context.obj_list().get(recreated_frame).map(|frame| frame.collect_data(context.objects()));

        Some(SetFrameTime {
            frame: self.frame,
            new_time: old_time,
            frame_recreation_ptr: recreated_frame,
            frame_recreation_data: recreate_data,
        })
    }

    #[cfg(debug_assertions)]
    fn debug_info(&self) -> String {
        let recreation_info = if let Some(recreate) = &self.frame_recreation_data {
            format!(" To recreate frame {} at time {}.", self.frame_recreation_ptr.key(), recreate.time)
        } else {
            String::new()
        };
        format!("Frame {} to time {}.{}", self.frame.key(), self.new_time, recreation_info)
    }

}
