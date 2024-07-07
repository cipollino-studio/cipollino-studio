use super::obj::{Obj, ObjList};


include!("layer.gen.rs");

impl Obj for Layer {

    fn obj_list(project: &Project) -> &ObjList<Self> {
        &project.layers
    }

    fn obj_list_mut(project: &mut Project) -> &mut ObjList<Self> {
        &mut project.layers
    }

}