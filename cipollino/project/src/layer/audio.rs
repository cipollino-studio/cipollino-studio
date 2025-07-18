
use crate::{AudioInstance, Objects, Project};

use super::{LayerParent, LayerPtr};

#[derive(alisa::Serializable, Clone)]
pub struct AudioLayer {
    pub parent: LayerParent,
    pub name: String,
    pub audio_instances: alisa::UnorderedChildList<alisa::OwningPtr<AudioInstance>>,
}

impl Default for AudioLayer {

    fn default() -> Self {
        Self {
            parent: LayerParent::Clip(alisa::Ptr::null()),
            name: "Audio".to_string(),
            audio_instances: Default::default()
        }
    }

}

impl alisa::Object for AudioLayer {

    type Project = Project;

    const TYPE_ID: u16 = 11;

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.audio_layers
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.audio_layers
    }

}

#[derive(alisa::Serializable)]
pub struct AudioLayerTreeData {
    pub name: String,
    pub audio_instances: alisa::UnorderedChildListTreeData<alisa::OwningPtr<AudioInstance>>
}

impl Default for AudioLayerTreeData {

    fn default() -> Self {
        Self {
            name: "Audio".to_owned(),
            audio_instances: Default::default()
        }
    }

}

impl alisa::TreeObj for AudioLayer {

    type ParentPtr = LayerParent;
    type ChildList = alisa::ChildList<LayerPtr>;
    type TreeData = AudioLayerTreeData;

    fn child_list<'a>(parent: Self::ParentPtr, context: &'a alisa::ProjectContext<Self::Project>) -> Option<&'a Self::ChildList> {
        parent.child_list(context)
    }

    fn child_list_mut<'a>(parent: Self::ParentPtr, recorder: &'a mut alisa::Recorder<Self::Project>) -> Option<&'a mut Self::ChildList> {
        parent.child_list_mut(recorder)
    }

    fn parent(&self) -> Self::ParentPtr {
        self.parent
    }

    fn parent_mut(&mut self) -> &mut Self::ParentPtr {
        &mut self.parent
    }

    fn instance(data: &Self::TreeData, ptr: alisa::Ptr<Self>, parent: Self::ParentPtr, recorder: &mut alisa::Recorder<Self::Project>) {
        let audio_layer = AudioLayer {
            parent,
            name: data.name.clone(),
            audio_instances: data.audio_instances.instance(ptr, recorder)
        };
        recorder.add_obj(ptr, audio_layer);
    }

    fn collect_data(&self, objects: &Objects) -> Self::TreeData {
        AudioLayerTreeData {
            name: self.name.clone(),
            audio_instances: self.audio_instances.collect_data(objects)
        }
    }

}

alisa::tree_object_operations!(AudioLayer);
alisa::object_set_property_operation!(AudioLayer, name, String);
