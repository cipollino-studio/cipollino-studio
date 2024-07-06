
use super::obj::{Obj, ObjList};

include!("clip.gen.rs");

impl Obj for Clip {

    fn obj_list(project: &Project) -> &ObjList<Self> {
        &project.clips
    }

    fn obj_list_mut(project: &mut Project) -> &mut ObjList<Self> {
        &mut project.clips
    }

}
