
use crate::{DeleteObjectDelta, Project, Recorder, RecreateObjectDelta, Serializable};

mod ptr;
pub use ptr::*;

mod obj_list;
pub use obj_list::*;

mod obj_kind;
pub use obj_kind::*;

pub trait Object: Sized + Clone + Serializable<Self::Project> + 'static {

    type Project: Project;

    const NAME: &'static str;

    fn list(objects: &<Self::Project as Project>::Objects) -> &ObjList<Self>;
    fn list_mut(objects: &mut <Self::Project as Project>::Objects) -> &mut ObjList<Self>;

    fn add(recorder: &mut Recorder<Self::Project>, ptr: Ptr<Self>, obj: Self) {
        recorder.obj_list_mut().insert(ptr, obj);
        recorder.push_delta(DeleteObjectDelta {
            ptr
        });
    }

    fn delete(recorder: &mut Recorder<Self::Project>, ptr: Ptr<Self>) {
        if let Some(obj) = recorder.obj_list_mut().delete(ptr) {
            recorder.push_delta(RecreateObjectDelta {
                ptr,
                obj,
            });
        }
    }

}