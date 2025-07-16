
use crate::{LoadingPtr, OwningPtr, Project, Ptr, Recorder, Serializable};

use super::TreeObj;

pub trait ChildPtr: Serializable + Clone + Eq {
    type ParentPtr: Clone; 
    type TreeData: Serializable;
    type Project: Project;

    fn collect_data(&self, objects: &<Self::Project as Project>::Objects) -> Option<Self::TreeData>;
    fn destroy(&self, recorder: &mut Recorder<Self::Project>);
    fn instance(&self, data: &Self::TreeData, parent: Self::ParentPtr, recorder: &mut Recorder<Self::Project>);

}

impl<O: TreeObj> ChildPtr for Ptr<O> {
    type ParentPtr = O::ParentPtr;
    type TreeData = O::TreeData;
    type Project = O::Project;
    
    fn collect_data(&self, objects: &<Self::Project as Project>::Objects) -> Option<Self::TreeData> {
        O::list(objects).get(*self).map(|obj| obj.collect_data(objects))
    }
    
    fn destroy(&self, recorder: &mut Recorder<Self::Project>) {
        if let Some(obj) = recorder.delete_obj(*self) {
            obj.destroy(recorder);
        }
    }
    
    fn instance(&self, data: &Self::TreeData, parent: Self::ParentPtr, recorder: &mut Recorder<Self::Project>) {
        O::instance(data, *self, parent, recorder);
    }

} 

impl<O: TreeObj> ChildPtr for LoadingPtr<O> {
    type ParentPtr = O::ParentPtr;
    type TreeData = O::TreeData;
    type Project = O::Project;
    
    fn collect_data(&self, objects: &<Self::Project as Project>::Objects) -> Option<Self::TreeData> {
        self.ptr().collect_data(objects)
    }
    
    fn destroy(&self, recorder: &mut Recorder<Self::Project>) {
        self.ptr().destroy(recorder);
    }
    
    fn instance(&self, data: &Self::TreeData, parent: Self::ParentPtr, recorder: &mut Recorder<Self::Project>) {
        self.ptr().instance(data, parent, recorder);
    }
}

impl<O: TreeObj> ChildPtr for OwningPtr<O> {
    type ParentPtr = O::ParentPtr;
    type TreeData = O::TreeData;
    type Project = O::Project;
    
    fn collect_data(&self, objects: &<Self::Project as Project>::Objects) -> Option<Self::TreeData> {
        self.ptr().collect_data(objects)
    }
    
    fn destroy(&self, recorder: &mut Recorder<Self::Project>) {
        self.ptr().destroy(recorder);
    }
    
    fn instance(&self, data: &Self::TreeData, parent: Self::ParentPtr, recorder: &mut Recorder<Self::Project>) {
        self.ptr().instance(data, parent, recorder);
    }
}
