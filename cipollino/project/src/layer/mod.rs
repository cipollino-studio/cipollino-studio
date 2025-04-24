
use crate::{Clip, Project};

mod layer;
pub use layer::*;

alisa::ptr_enum!(LayerParent [Clip]);

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
                .and_then(|clip| context.obj_list().get(clip.inner))
                .map(|inner| &inner.layers),
        }
    }

    fn child_list_mut<'a>(&self, recorder: &'a mut alisa::Recorder<Project>) -> Option<&'a mut alisa::ChildList<LayerPtr>> {
        match self {
            LayerParent::Clip(ptr) => recorder.get_obj(*ptr)
                .map(|clip| clip.inner)
                .and_then(|inner| recorder.get_obj_mut(inner))
                .map(|inner| &mut inner.layers),
        }
    }

}

alisa::ptr_enum!(LayerPtr loading [Layer] childof LayerParent, in Project);
