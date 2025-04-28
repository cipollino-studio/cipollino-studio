
use crate::{Layer, Objects, Project, SceneObjPtr};

mod creation;
pub use creation::*;

mod set_time;
pub use set_time::*;

#[derive(alisa::Serializable, Clone)]
pub struct Frame {
    pub layer: alisa::Ptr<Layer>,
    pub time: i32,
    pub scene: alisa::ChildList<SceneObjPtr> 
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            layer: alisa::Ptr::null(),
            time: 0,
            scene: alisa::ChildList::new()
        }
    }
}

impl alisa::Object for Frame {
    type Project = Project;

    const NAME: &'static str = "Frame";
    const TYPE_ID: u16 = 1;

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.frames
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.frames
    }

}

#[derive(alisa::Serializable)]
pub struct FrameTreeData {
    pub time: i32,
    pub scene: alisa::ChildListTreeData<SceneObjPtr>
}

impl Default for FrameTreeData {

    fn default() -> Self {
        Self {
            time: 0,
            scene: Default::default()
        }
    }

}

impl alisa::TreeObj for Frame {
    type ParentPtr = alisa::Ptr<Layer>;
    type ChildList = alisa::UnorderedChildList<alisa::LoadingPtr<Self>>;
    type TreeData = FrameTreeData;

    fn child_list<'a>(parent: alisa::Ptr<Layer>, context: &'a alisa::ProjectContext<Project>) -> Option<&'a Self::ChildList> {
        Some(&context.obj_list().get(parent)?.frames)
    }

    fn child_list_mut<'a>(parent: alisa::Ptr<Layer>, context: &'a mut alisa::Recorder<Project>) -> Option<&'a mut Self::ChildList> {
        Some(&mut context.get_obj_mut(parent)?.frames)
    }

    fn parent(&self) -> alisa::Ptr<Layer> {
        self.layer
    }

    fn parent_mut(&mut self) -> &mut alisa::Ptr<Layer> {
        &mut self.layer
    }

    fn instance(data: &FrameTreeData, ptr: alisa::Ptr<Self>, parent: alisa::Ptr<Layer>, recorder: &mut alisa::Recorder<Project>) {
        let frame = Frame {
            layer: parent,
            time: data.time,
            scene: data.scene.instance(ptr, recorder),
        };
        recorder.add_obj(ptr, frame);
    }

    fn destroy(&self, recorder: &mut alisa::Recorder<Self::Project>) {
        self.scene.destroy(recorder);
    }

    fn collect_data(&self, objects: &Objects) -> FrameTreeData {
        FrameTreeData {
            time: self.time,
            scene: self.scene.collect_data(objects),
        }
    }

}

fn find_frame_at_time(context: &alisa::ProjectContext<'_, Project>, frames: &alisa::UnorderedChildList<alisa::LoadingPtr<Frame>>, time: i32) -> Option<alisa::Ptr<Frame>> {
    for frame_ptr in frames.iter() {
        if let Some(frame) = context.obj_list().get(frame_ptr.ptr()) {
            if frame.time == time {
                return Some(frame_ptr.ptr());
            }
        }
    }
    None
}