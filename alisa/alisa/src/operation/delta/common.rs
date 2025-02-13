
use crate::{Delta, Ptr, Object, ProjectContextMut};

pub struct DeleteObjectDelta<O: Object> {
    pub ptr: Ptr<O>
} 

impl<O: Object> Delta for DeleteObjectDelta<O> {
    type Project = O::Project;

    fn perform(&self, context: &mut ProjectContextMut<O::Project>) {
        context.obj_list_mut().delete(self.ptr);
    }
}

pub struct RecreateObjectDelta<O: Object> {
    pub ptr: Ptr<O>,
    pub obj: O
}

impl<O: Object> Delta for RecreateObjectDelta<O> {
    type Project = O::Project;

    fn perform(&self, context: &mut ProjectContextMut<O::Project>) {
        context.obj_list_mut().insert(self.ptr, self.obj.clone());
    }
} 
