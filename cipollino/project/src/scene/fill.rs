
use crate::{Frame, Objects, Project, SceneObjectColor};

use super::SceneObjPtr;

#[derive(Clone, Default)]
pub struct FillPaths(pub malvina::FillPaths);

fn encode_path(path: &elic::BezierPath<elic::Vec2>) -> Vec<u8> {
    let mut bytes = Vec::new();
    for pt in path.pts.iter() {
        bytes.extend_from_slice(&pt.prev.x.to_le_bytes());
        bytes.extend_from_slice(&pt.prev.y.to_le_bytes());
        bytes.extend_from_slice(&pt.pt.x.to_le_bytes());
        bytes.extend_from_slice(&pt.pt.y.to_le_bytes());
        bytes.extend_from_slice(&pt.next.x.to_le_bytes());
        bytes.extend_from_slice(&pt.next.y.to_le_bytes());
    }
    bytes
}

fn decode_path(data: &[u8]) -> elic::BezierPath<elic::Vec2> {
    let floats: Box<[f32]> = data.chunks(4).filter_map(|x| {
        if x.len() != 4 {
            None
        } else {
            Some(f32::from_le_bytes([x[0], x[1], x[2], x[3]]))
        }
    }).collect();
    elic::BezierPath {
        pts: floats.chunks(6).filter_map(|floats| {
            if floats.len() < 6 {
                return None;
            }
            Some(elic::BezierPoint {
                prev: elic::vec2(floats[0], floats[1]),
                pt: elic::vec2(floats[2], floats[3]),
                next: elic::vec2(floats[4], floats[5]),
            })
        }).collect(),
    }
}

impl alisa::Serializable for FillPaths {

    fn serialize(&self, _context: &alisa::SerializationContext) -> alisa::ABFValue {
        alisa::ABFValue::Array(
            self.0.paths.iter().map(|path| alisa::ABFValue::Binary(encode_path(path).into_boxed_slice())).collect()
        )
    }

    fn deserialize(data: &alisa::ABFValue, _context: &mut alisa::DeserializationContext) -> Option<Self> {
        let paths = data.as_array()?;
        Some(Self(
            malvina::FillPaths {
                paths: paths.iter().filter_map(|path_data| Some(decode_path(path_data.as_binary()?))).collect()
            }
        ))
    }

    fn delete(&self, _: &mut Vec<alisa::AnyPtr>) {
        
    }

}

#[derive(Clone, alisa::Serializable)]
pub struct Fill {
    pub frame: alisa::Ptr<Frame>,
    pub paths: FillPaths,
    pub color: SceneObjectColor 
}

impl Default for Fill {
    fn default() -> Self {
        Self {
            frame: Default::default(),
            paths: Default::default(),
            color: Default::default() 
        }
    }
}

impl alisa::Object for Fill {

    type Project = Project;
    const TYPE_ID: u16 = 7;

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.fills
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.fills
    }

}

#[derive(alisa::Serializable)]
pub struct FillTreeData {
    pub paths: FillPaths,
    pub color: SceneObjectColor 
}

impl Default for FillTreeData {

    fn default() -> Self {
        Self {
            paths: Default::default(),
            color: Default::default()
        }
    }

}

impl alisa::TreeObj for Fill {
    type ParentPtr = alisa::Ptr<Frame>;
    type ChildList = alisa::ChildList<SceneObjPtr>;
    type TreeData = FillTreeData;

    fn child_list<'a>(parent: Self::ParentPtr, context: &'a alisa::ProjectContext<Self::Project>) -> Option<&'a Self::ChildList> {
        context.obj_list().get(parent).map(|frame| &frame.scene)
    }

    fn child_list_mut<'a>(parent: Self::ParentPtr, recorder: &'a mut alisa::Recorder<Self::Project>) -> Option<&'a mut Self::ChildList> {
        recorder.get_obj_mut(parent).map(|frame| &mut frame.scene)
    }

    fn parent(&self) -> Self::ParentPtr {
        self.frame
    }

    fn parent_mut(&mut self) -> &mut Self::ParentPtr {
        &mut self.frame
    }

    fn instance(data: &Self::TreeData, ptr: alisa::Ptr<Self>, frame: Self::ParentPtr, recorder: &mut alisa::Recorder<Project>) {
        let fill = Fill {
            frame,
            paths: data.paths.clone(),
            color: data.color,
        };
        recorder.add_obj(ptr, fill);
    }

    fn collect_data(&self, _objects: &Objects) -> Self::TreeData {
        FillTreeData {
            paths: self.paths.clone(),
            color: self.color
        }
    }

}

alisa::tree_object_operations!(Fill);
alisa::object_set_property_operation!(Fill, paths, FillPaths);
alisa::object_set_property_operation!(Fill, color, SceneObjectColor);
