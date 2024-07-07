use super::obj::{Obj, ObjList};


include!("frame.gen.rs");

impl Obj for Frame {

    fn obj_list(project: &Project) -> &ObjList<Self> {
        &project.frames
    }

    fn obj_list_mut(project: &mut Project) -> &mut ObjList<Self> {
        &mut project.frames
    }

}