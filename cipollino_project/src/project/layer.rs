use super::obj::{Obj, ObjList, ObjRef};


include!("layer.gen.rs");

impl Obj for Layer {

    fn obj_list(project: &Project) -> &ObjList<Self> {
        &project.layers
    }

    fn obj_list_mut(project: &mut Project) -> &mut ObjList<Self> {
        &mut project.layers
    }

}

impl Layer {

    pub fn find_frame_exactly_at<'a>(&'a self, frames: &'a ObjList<Frame>, time: i32) -> Option<ObjRef<'a, Frame>> {
        for frame in self.frames.iter_ref(&frames) {
            if *frame.time.value() == time {
                return Some(frame);
            }
        }
        None 
    }

}
