
use crate::{Frame, Objects, Project};

use super::SceneChildPtr;

#[derive(Clone, Default)]
pub struct StrokeData(pub malvina::Stroke);

fn encode_stroke_point(buffer: &mut Vec<u8>, pt: &malvina::StrokePoint) {
    buffer.extend_from_slice(&pt.pt.x.to_bits().to_le_bytes());
    buffer.extend_from_slice(&pt.pt.y.to_bits().to_le_bytes());
    buffer.extend_from_slice(&pt.pressure.to_bits().to_le_bytes());
}

fn decode_stroke_point(floats: &[f32]) -> Option<malvina::StrokePoint> {
    if floats.len() != 3 {
        return None;
    }
    Some(malvina::StrokePoint {
        pt: malvina::vec2(floats[0], floats[1]),
        pressure: floats[2]
    })
}

impl<P: alisa::Project> alisa::Serializable<P> for StrokeData {

    fn serialize(&self, _context: &alisa::SerializationContext<P>) -> alisa::rmpv::Value {
        let stroke = &self.0;
        let mut buffer = Vec::new();
        for pt in &stroke.path.pts {
            encode_stroke_point(&mut buffer, &pt.prev);
            encode_stroke_point(&mut buffer, &pt.pt);
            encode_stroke_point(&mut buffer, &pt.next);
        }
        alisa::rmpv::Value::Binary(buffer)
    }

    fn deserialize(data: &alisa::rmpv::Value, _context: &mut alisa::DeserializationContext<P>) -> Option<Self> {
        if !data.is_bin() {
            return None;
        }
        let bytes = data.as_slice()?;
        let floats: Box<[f32]> = bytes.chunks(4).filter_map(|x| {
            if x.len() != 4 {
                None
            } else {
                Some(u32::from_le_bytes([x[0], x[1], x[2], x[3]]))
            }
        }).map(|bits| f32::from_bits(bits)).collect();
        let stroke_points: Box<[malvina::StrokePoint]> = floats.chunks(3).filter_map(|x| decode_stroke_point(x)).collect();
        let bezier_points = stroke_points.chunks(3).filter_map(|pts| {
            if pts.len() != 3 {
                return None;
            }
            Some(elic::BezierPoint {
                prev: pts[0],
                pt: pts[1],
                next: pts[2],
            })
        }).collect();
        Some(Self(malvina::Stroke {
            path: elic::BezierPath { pts: bezier_points },
        }))
    }

}

#[derive(Clone, alisa::Serializable)]
#[project(Project)]
pub struct Stroke {
    pub frame: alisa::Ptr<Frame>,
    pub stroke: StrokeData,
    pub color: [f32; 4],
    pub width: f32
} 

impl Default for Stroke {

    fn default() -> Self {
        Self {
            frame: alisa::Ptr::null(),
            stroke: StrokeData(malvina::Stroke::empty()),
            color: [0.0, 0.0, 0.0, 1.0],
            width: 5.0,
        }
    }

}

impl alisa::Object for Stroke {
    type Project = Project;
    const NAME: &'static str = "Stroke";

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.strokes
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.strokes
    }
}

#[derive(alisa::Serializable)]
pub struct StrokeTreeData {
    pub stroke: StrokeData,
    pub color: [f32; 4],
    pub width: f32
}

impl Default for StrokeTreeData {

    fn default() -> Self {
        Self {
            stroke: StrokeData(malvina::Stroke::empty()),
            color: [0.0, 0.0, 0.0, 1.0],
            width: 5.0
        }
    }

}

impl alisa::TreeObj for Stroke {
    type ParentPtr = alisa::Ptr<Frame>;
    type ChildList = alisa::ChildList<SceneChildPtr>;
    type TreeData = StrokeTreeData;

    fn child_list<'a>(parent: alisa::Ptr<Frame>, context: &'a alisa::ProjectContext<Project>) -> Option<&'a Self::ChildList> {
        context.obj_list().get(parent).map(|frame| &frame.scene)
    }

    fn child_list_mut<'a>(parent: alisa::Ptr<Frame>, context: &'a mut alisa::Recorder<Project>) -> Option<&'a mut Self::ChildList> {
        context.get_obj_mut(parent).map(|frame| &mut frame.scene)
    }

    fn parent(&self) -> alisa::Ptr<Frame> {
        self.frame
    }

    fn parent_mut(&mut self) -> &mut alisa::Ptr<Frame> {
        &mut self.frame
    }

    fn instance(data: &StrokeTreeData, ptr: alisa::Ptr<Stroke>, frame: alisa::Ptr<Frame>, recorder: &mut alisa::Recorder<Project>) {
        let stroke = Stroke {
            frame,
            stroke: data.stroke.clone(),
            color: data.color,
            width: data.width
        };
        recorder.add_obj(ptr, stroke);
    }

    fn destroy(&self, _recorder: &mut alisa::Recorder<Project>) {

    }

    fn collect_data(&self, _objects: &Objects) -> StrokeTreeData {
        StrokeTreeData {
            stroke: self.stroke.clone(),
            color: self.color,
            width: self.width
        }
    }

}

alisa::tree_object_operations!(Stroke);
alisa::object_set_property_operation!(Stroke, stroke, StrokeData);
alisa::object_set_property_operation!(Stroke, color, [f32; 4]);
