
use crate::{Objects, Project};

use super::{LayerParent, LayerPtr};

#[derive(alisa::Serializable, Clone)]
pub struct LayerGroup {
    pub parent: LayerParent,
    pub name: String,
    pub layers: alisa::ChildList<LayerPtr>
}

impl Default for LayerGroup {

    fn default() -> Self {
        Self {
            parent: LayerParent::Clip(alisa::Ptr::null()),
            name: "Layer Group".to_owned(),
            layers: alisa::ChildList::new() 
        }
    }
    
}

impl alisa::Object for LayerGroup {
    type Project = Project;
    const TYPE_ID: u16 = 6;

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.layer_groups
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.layer_groups
    }
}

#[derive(alisa::Serializable)]
pub struct LayerGroupTreeData {
    pub name: String,
    pub layers: alisa::ChildListTreeData<LayerPtr>
}

impl Default for LayerGroupTreeData {

    fn default() -> Self {
        Self {
            name: "Layer Group".to_string(),
            layers: alisa::ChildListTreeData::default() 
        }
    }

}

impl alisa::TreeObj for LayerGroup {
    type ParentPtr = LayerParent;
    type ChildList = alisa::ChildList<LayerPtr>;
    type TreeData = LayerGroupTreeData;

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

    fn instance(data: &Self::TreeData, ptr: alisa::Ptr<Self>, parent: Self::ParentPtr, recorder: &mut alisa::Recorder<Project>) {
        let layer_group = LayerGroup {
            parent,
            name: data.name.clone(),
            layers: data.layers.instance(ptr.into(), recorder),
        };
        recorder.add_obj(ptr, layer_group);
    }

    fn destroy(&self, _recorder: &mut alisa::Recorder<Project>) {

    }

    fn collect_data(&self, objects: &Objects) -> Self::TreeData {
        LayerGroupTreeData {
            name: self.name.clone(),
            layers: self.layers.collect_data(objects),
        }
    }
    
}

alisa::tree_object_creation_operations!(LayerGroup);
alisa::object_set_property_operation!(LayerGroup, name, String);

/*
    We need a custom transfer operation to account for the possibility that a layer group 
    could be transferred into a child layer group, which should be impossible.
*/
#[derive(alisa::Serializable)]
pub struct TransferLayerGroup {
    pub ptr: alisa::Ptr<LayerGroup>,
    pub new_parent: LayerParent,
    pub new_idx: usize
}

impl Default for TransferLayerGroup {

    fn default() -> Self {
        Self {
            ptr: alisa::Ptr::null(),
            new_parent: LayerParent::Clip(alisa::Ptr::null()),
            new_idx: 0
        }
    }

}

fn is_inside_layer_group(recorder: &alisa::Recorder<Project>, parent_ptr: alisa::Ptr<LayerGroup>, child_ptr: LayerParent) -> bool {
    match child_ptr {
        LayerParent::Clip(_) => false,
        LayerParent::LayerGroup(child_ptr) => {
            if parent_ptr == child_ptr {
                return true;
            }
            if let Some(child) = recorder.get_obj(child_ptr) {
                return is_inside_layer_group(recorder, parent_ptr, child.parent);
            } 
            false
        }
    }
}

impl alisa::Operation for TransferLayerGroup {

    type Project = Project;
    const NAME: &'static str = "TransferLayerGroup";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Project>) -> bool {
        // Make sure we're not moving the layer group somewhere inside itself
        if is_inside_layer_group(recorder, self.ptr, self.new_parent) {
            return false;
        }

        if alisa::transfer_tree_object(recorder, self.ptr, &self.new_parent, &self.new_idx) {
            true
        } else {
            false
        }

    }

}

impl alisa::InvertibleOperation for TransferLayerGroup {

    type Inverse = Self;

    fn inverse(&self, context: &alisa::ProjectContext<Project>) -> Option<Self> {
        use alisa::Children;
        use alisa::TreeObj;
        let layer_group = context.obj_list().get(self.ptr)?;
        let parent = layer_group.parent;
        let child_list = LayerGroup::child_list(parent, context)?; 
        let idx = child_list.index_of(self.ptr)?;
        Some(Self {
            ptr: self.ptr,
            new_parent: layer_group.parent,
            new_idx: idx
        })
    }

}
