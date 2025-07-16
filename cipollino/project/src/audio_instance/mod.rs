
use crate::{AudioClip, AudioLayer, Objects, Project};

mod operations;
pub use operations::*;

#[derive(Default, Clone, alisa::Serializable)]
pub struct AudioInstance {
    pub layer: alisa::Ptr<AudioLayer>,
    pub clip: alisa::Ptr<AudioClip>,
    // The start of the audio instance on the timeline, in seconds
    pub start: f32,
    // The end of the audio instance on the timeline, in seconds
    pub end: f32,
    // The time at which we start playing the audio clip, in seconds 
    pub offset: f32
}

impl alisa::Object for AudioInstance {
    type Project = Project;
    const TYPE_ID: u16 = 14;

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.audio_instances
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.audio_instances
    }
}

#[derive(Default, alisa::Serializable)]
pub struct AudioInstanceTreeData {
    pub clip: alisa::Ptr<AudioClip>,
    pub start: f32,
    pub end: f32,
    pub offset: f32
}

impl alisa::TreeObj for AudioInstance {
    type ParentPtr = alisa::Ptr<AudioLayer>;
    type ChildList = alisa::UnorderedChildList<alisa::OwningPtr<AudioInstance>>;
    type TreeData = AudioInstanceTreeData;

    fn child_list<'a>(parent: Self::ParentPtr, context: &'a alisa::ProjectContext<Self::Project>) -> Option<&'a Self::ChildList> {
        Some(&context.obj_list().get(parent)?.audio_instances)
    }

    fn child_list_mut<'a>(parent: Self::ParentPtr, recorder: &'a mut alisa::Recorder<Self::Project>) -> Option<&'a mut Self::ChildList> {
        Some(&mut recorder.get_obj_mut(parent)?.audio_instances)
    }

    fn parent(&self) -> Self::ParentPtr {
        self.layer
    }

    fn parent_mut(&mut self) -> &mut Self::ParentPtr {
        &mut self.layer
    }

    fn instance(data: &Self::TreeData, ptr: alisa::Ptr<Self>, parent: Self::ParentPtr, recorder: &mut alisa::Recorder<Self::Project>) {
        recorder.add_obj(ptr, AudioInstance {
            layer: parent,
            clip: data.clip,
            start: data.start,
            end: data.end,
            offset: data.offset 
        });
    }

    fn destroy(&self, _recorder: &mut alisa::Recorder<Self::Project>) {

    }

    fn collect_data(&self, _objects: &Objects) -> Self::TreeData {
        AudioInstanceTreeData {
            clip: self.clip,
            start: self.start,
            end: self.end,
            offset: self.offset
        }
    }

}

impl AudioInstance {

    pub fn length(&self) -> f32 {
        self.end - self.start
    }

}

alisa::object_set_property_operation!(AudioInstance, offset, f32);
