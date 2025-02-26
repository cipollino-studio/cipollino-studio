
use crate::{Layer, Objects, Project};

mod creation;
pub use creation::*;

#[derive(alisa::Serializable, Clone)]
#[project(Project)]
pub struct Frame {
    pub layer: alisa::Ptr<Layer>,
    pub time: i32
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            layer: alisa::Ptr::null(),
            time: 0
        }
    }
}

impl alisa::Object for Frame {
    type Project = Project;

    const NAME: &'static str = "Frame";

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.frames
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.frames
    }

}

#[derive(alisa::Serializable)]
pub struct FrameTreeData {
    pub time: i32
}

impl Default for FrameTreeData {

    fn default() -> Self {
        Self {
            time: 0
        }
    }

}

impl alisa::TreeObj for Frame {
    type ParentPtr = alisa::Ptr<Layer>;
    type ChildList = alisa::UnorderedChildList<Self>;
    type TreeData = FrameTreeData;

    fn child_list<'a>(parent: alisa::Ptr<Layer>, context: &'a alisa::ProjectContext<Project>) -> Option<&'a Self::ChildList> {
        Some(&context.obj_list().get(parent)?.frames)
    }

    fn child_list_mut<'a>(parent: alisa::Ptr<Layer>, context: &'a mut alisa::ProjectContextMut<Project>) -> Option<&'a mut Self::ChildList> {
        Some(&mut context.obj_list_mut().get_mut(parent)?.frames)
    }

    fn parent(&self) -> alisa::Ptr<Layer> {
        self.layer
    }

    fn parent_mut(&mut self) -> &mut alisa::Ptr<Layer> {
        &mut self.layer
    }

    fn instance(data: &FrameTreeData, ptr: alisa::Ptr<Self>, parent: alisa::Ptr<Layer>, recorder: &mut alisa::Recorder<Project>) {
        use alisa::Object;
        let frame = Frame {
            layer: parent,
            time: data.time,
        };
        Self::add(recorder, ptr, frame);
    }

    fn destroy(&self, _recorder: &mut alisa::Recorder<Self::Project>) {

    }

    fn collect_data(&self, _objects: &<Self::Project as alisa::Project>::Objects) -> FrameTreeData {
        FrameTreeData {
            time: self.time,
        }
    }

}
