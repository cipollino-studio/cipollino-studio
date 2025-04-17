

use crate::{frame::find_frame_at_time, Layer, Project};
use super::{Frame, FrameTreeData};

#[derive(alisa::Serializable, Default)]
pub struct CreateFrame {
    pub ptr: alisa::Ptr<Frame>,
    pub layer: alisa::Ptr<Layer>,
    pub data: FrameTreeData
}

#[derive(alisa::Serializable, Default)]
pub struct DeleteFrame {
    pub ptr: alisa::Ptr<Frame>
}


impl alisa::Operation for CreateFrame {
    type Project = Project;

    const NAME: &'static str = "CreateFrame";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Project>) -> bool {

        use alisa::Children;
        use alisa::TreeObj;

        // Make sure the parent we're creating the object in exists 
        let context = recorder.context();
        let Some(frames) = Frame::child_list(self.layer, &context) else { return false; };

        // If there's already a frame here, do nothing
        if find_frame_at_time(&context, frames, self.data.time).is_some() {
            return true;
        }

        // Instance the frame and its children
        Frame::instance(&self.data, self.ptr, self.layer, recorder);

        // Add it to the layer's frame list
        let Some(frames) = Frame::child_list_mut(self.layer, recorder) else { return false; };
        frames.insert((), self.ptr.into());

        true

    }

}

impl alisa::InvertibleOperation for CreateFrame {

    type Inverse = DeleteFrame;

    fn inverse(&self, context: &alisa::ProjectContext<Project>) -> Option<Self::Inverse> {
        use alisa::TreeObj;

        // If a frame at this time already exists, there's nothing to undo 
        let frames = Frame::child_list(self.layer, context)?; 
        if find_frame_at_time(context, frames, self.data.time).is_some() {
            return None;
        }

        Some(DeleteFrame {
            ptr: self.ptr,
        })
    }

}

impl alisa::Operation for DeleteFrame {
    type Project = Project;

    const NAME: &'static str = "DeleteFrame";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Project>) -> bool {
        alisa::delete_tree_object(recorder, self.ptr)
    }

    

}

impl alisa::InvertibleOperation for DeleteFrame {
    type Inverse = CreateFrame;

    fn inverse(&self, context: &alisa::ProjectContext<Project>) -> Option<Self::Inverse> {
        use alisa::TreeObj;
        let frame = context.obj_list().get(self.ptr)?; 
        let data = frame.collect_data(context.objects()); 
        let layer = frame.layer;
        Some(CreateFrame {
            ptr: self.ptr,
            layer,
            data,
        })
    }
}