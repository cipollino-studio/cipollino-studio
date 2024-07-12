
use super::obj::{Obj, ObjList};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct StrokePoint {

}

impl ObjSerialize for StrokePoint {

    fn obj_serialize(&self, _project: &Project, _serializer: &mut Serializer) -> bson::Bson {
        bson::to_bson(self).unwrap()
    }

    fn obj_deserialize(_project: &mut Project, data: &bson::Bson, _serializer: &mut Serializer, _idx: FractionalIndex) -> Option<Self> {
        bson::from_bson(data.clone()).ok()?
    }

}

include!("stroke.gen.rs");

impl Obj for Stroke {

    fn obj_list(project: &Project) -> &ObjList<Self> {
        &project.strokes
    }

    fn obj_list_mut(project: &mut Project) -> &mut ObjList<Self> {
        &mut project.strokes
    }

}
