
use crate::{Clip, Project};

mod layer;
pub use layer::*;

mod audio;
pub use audio::*;

mod group;
pub use group::*;

alisa::ptr_enum!(LayerParent [Clip, LayerGroup]);

impl Default for LayerParent {
    fn default() -> Self {
        Self::Clip(alisa::Ptr::null())
    }
}

impl LayerParent {

    fn child_list<'a>(&self, context: &'a alisa::ProjectContext<Project>) -> Option<&'a alisa::ChildList<LayerPtr>> {
        match self {
            LayerParent::Clip(ptr) => context.obj_list()
                .get(*ptr)
                .and_then(|clip| context.obj_list().get(clip.inner.ptr()))
                .map(|inner| &inner.layers),
            LayerParent::LayerGroup(ptr) => context.obj_list()
                .get(*ptr)
                .map(|group| &group.layers),
        }
    }

    fn child_list_mut<'a>(&self, recorder: &'a mut alisa::Recorder<Project>) -> Option<&'a mut alisa::ChildList<LayerPtr>> {
        match self {
            LayerParent::Clip(ptr) => recorder.get_obj(*ptr)
                .map(|clip| clip.inner)
                .and_then(|inner| recorder.get_obj_mut(inner.ptr()))
                .map(|inner| &mut inner.layers),
            LayerParent::LayerGroup(ptr) => recorder.get_obj_mut(*ptr)
                .map(|group| &mut group.layers)
        }
    }

}

alisa::ptr_enum!(LayerPtr owning [Layer, AudioLayer, LayerGroup] childof LayerParent, in Project);
