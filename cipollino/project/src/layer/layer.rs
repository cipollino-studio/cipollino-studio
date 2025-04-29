
use crate::{Client, Frame, Objects, Project};
use super::{LayerPtr, LayerParent};


#[derive(alisa::Serializable, Clone)]
pub struct Layer {
    pub parent: LayerParent,

    pub name: String,

    pub frames: alisa::UnorderedChildList<alisa::LoadingPtr<Frame>>
}

impl Default for Layer {

    fn default() -> Self {
        Self {
            parent: LayerParent::Clip(alisa::Ptr::null()),
            name: "Layer".to_owned(),
            frames: alisa::UnorderedChildList::new()
        }
    }

}

impl alisa::Object for Layer {
    type Project = Project;

    const TYPE_ID: u16 = 2;

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.layers
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.layers
    }
}

#[derive(alisa::Serializable)]
pub struct LayerTreeData {
    pub name: String,
    pub frames: alisa::UnorderedChildListTreeData<alisa::LoadingPtr<Frame>>
}

impl Default for LayerTreeData {
    fn default() -> Self {
        Self {
            name: "Layer".to_owned(),
            frames: alisa::UnorderedChildListTreeData::default()
        }
    }
}

impl alisa::TreeObj for Layer {
    type ParentPtr = LayerParent;
    type ChildList = alisa::ChildList<LayerPtr>;
    type TreeData = LayerTreeData;

    fn child_list<'a>(parent: LayerParent, context: &'a alisa::ProjectContext<Project>) -> Option<&'a Self::ChildList> {
        parent.child_list(context)
    }

    fn child_list_mut<'a>(parent: LayerParent, recorder: &'a mut alisa::Recorder<Project>) -> Option<&'a mut Self::ChildList> {
        parent.child_list_mut(recorder)
    }

    fn parent(&self) -> LayerParent {
        self.parent
    }

    fn parent_mut(&mut self) -> &mut LayerParent {
        &mut self.parent
    }

    fn instance(data: &Self::TreeData, ptr: alisa::Ptr<Self>, parent: Self::ParentPtr, recorder: &mut alisa::Recorder<Self::Project>) {
        let layer = Layer {
            parent,
            name: data.name.clone(),
            frames: data.frames.instance(ptr, recorder)
        };
        recorder.add_obj(ptr, layer);
    }

    fn destroy(&self, recorder: &mut alisa::Recorder<Self::Project>) {
        self.frames.destroy(recorder);
    }

    fn collect_data(&self, objects: &Objects) -> Self::TreeData {
        LayerTreeData {
            name: self.name.clone(),
            frames: self.frames.collect_data(objects)
        }
    }

}

alisa::tree_object_operations!(Layer);
alisa::object_set_property_operation!(Layer, name, String);

impl Layer { 

    pub fn frame_at(&self, client: &Client, t: i32) -> Option<alisa::Ptr<Frame>> {
        let mut max_frame = None;
        let mut max_time = i32::MIN;
        for frame_ptr in self.frames.iter() {
            if let Some(frame) = client.get(frame_ptr.ptr()) {
                if frame.time > max_time && frame.time <= t {
                    max_frame = Some(frame_ptr.ptr());
                    max_time = frame.time;
                }
            }
        }
        max_frame
    } 

    pub fn frame_before(&self, client: &Client, t: i32) -> Option<alisa::Ptr<Frame>> {
        self.frame_at(client, t - 1)
    }

    pub fn frame_after(&self, client: &Client, t: i32) -> Option<alisa::Ptr<Frame>> {
        let mut min_frame = None;
        let mut min_time = i32::MAX;
        for frame_ptr in self.frames.iter() {
            if let Some(frame) = client.get(frame_ptr.ptr()) {
                if frame.time < min_time && frame.time > t {
                    min_frame = Some(frame_ptr.ptr());
                    min_time = frame.time;
                }
            }
        }
        min_frame
    }

    pub fn frame_exactly_at(&self, client: &Client, t: i32) -> Option<alisa::Ptr<Frame>> {
        for frame_ptr in self.frames.iter() {
            if let Some(frame) = client.get(frame_ptr.ptr()) {
                if frame.time == t { 
                    return Some(frame_ptr.ptr());
                }
            }
        }
        None
    }

}
