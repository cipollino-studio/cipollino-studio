
use crate::{Objects, Project};
use super::{LayerChildPtr, LayerChildList, LayerParent, LayerType};


#[derive(alisa::Serializable, Clone)]
#[project(Project)]
pub struct Layer {
    pub parent: LayerParent,
    pub name: String
}

impl Default for Layer {

    fn default() -> Self {
        Self {
            parent: LayerParent::Clip(alisa::Ptr::null()),
            name: "Layer".to_owned() 
        }
    }

}

impl alisa::Object for Layer {
    type Project = Project;

    const NAME: &'static str = "Layer";

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.layers
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.layers
    }
}

#[derive(alisa::Serializable)]
pub struct LayerTreeData {
    pub name: String
}

impl Default for LayerTreeData {
    fn default() -> Self {
        Self {
            name: "Layer".to_owned()
        }
    }
}

impl alisa::TreeObj for Layer {
    type ParentPtr = LayerParent;
    type ChildList = LayerChildList;
    type TreeData = LayerTreeData;

    fn child_list<'a>(parent: LayerParent, context: &'a alisa::ProjectContext<Project>) -> Option<&'a Self::ChildList> {
        parent.child_list(context)
    }

    fn child_list_mut<'a>(parent: LayerParent, context: &'a mut alisa::ProjectContextMut<Project>) -> Option<&'a mut Self::ChildList> {
        parent.child_list_mut(context)
    }

    fn parent(&self) -> LayerParent {
        self.parent
    }

    fn parent_mut(&mut self) -> &mut LayerParent {
        &mut self.parent
    }

    fn instance(data: &Self::TreeData, ptr: alisa::Ptr<Self>, parent: Self::ParentPtr, recorder: &mut alisa::Recorder<Self::Project>) {
        use alisa::Object;
        let layer = Layer {
            parent,
            name: data.name.clone(),
        };
        Self::add(recorder, ptr, layer);
    }

    fn destroy(&self, _recorder: &mut alisa::Recorder<Self::Project>) {

    }

    fn collect_data(&self, _objects: &Objects) -> Self::TreeData {
        LayerTreeData {
            name: self.name.clone(),
        }
    }

}

impl LayerType for Layer {

    fn make_child_ptr(ptr: alisa::Ptr<Self>) -> LayerChildPtr {
        LayerChildPtr::Layer(alisa::LoadingPtr::new(ptr))
    }

}

alisa::tree_object_operations!(Layer);
alisa::object_set_property_operation!(Layer, name, String);
