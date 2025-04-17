
mod stroke;
pub use stroke::*;

use crate::{Frame, Objects, Project};

#[derive(Clone, Copy, PartialEq, Eq, alisa::Serializable)]
pub enum SceneChildPtr {
    Stroke(alisa::LoadingPtr<Stroke>)
}

impl From<alisa::Ptr<Stroke>> for SceneChildPtr {

    fn from(ptr: alisa::Ptr<Stroke>) -> Self {
        SceneChildPtr::Stroke(alisa::LoadingPtr::new(ptr))
    }

}

#[derive(alisa::Serializable)]
pub enum SceneChildTreeData {
    Stroke(StrokeTreeData)
}

impl alisa::ChildPtr for SceneChildPtr {
    type ParentPtr = alisa::Ptr<Frame>;
    type TreeData = SceneChildTreeData;
    type Project = Project;

    fn collect_data(&self, objects: &Objects) -> Option<Self::TreeData> {
        match self {
            SceneChildPtr::Stroke(ptr) => {
                ptr.collect_data(objects).map(SceneChildTreeData::Stroke)
            },
        }
    }

    fn destroy(&self, recorder: &mut alisa::Recorder<Self::Project>) {
        match self {
            SceneChildPtr::Stroke(ptr) => {
                ptr.destroy(recorder);
            },
        }
    }

    fn instance(&self, data: &Self::TreeData, parent: Self::ParentPtr, recorder: &mut alisa::Recorder<Self::Project>) {
        match data {
            SceneChildTreeData::Stroke(data) => {
                let SceneChildPtr::Stroke(ptr) = *self;
                ptr.instance(data, parent, recorder);
            },
        } 
    }
}
