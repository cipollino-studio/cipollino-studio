
mod brush;
pub use brush::*;

use crate::{Frame, Objects, Project, SceneObjectColor};

use super::SceneObjPtr;

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

impl alisa::Serializable for StrokeData {

    fn serialize(&self, _context: &alisa::SerializationContext) -> alisa::ABFValue {
        let stroke = &self.0;
        let mut buffer = Vec::new();
        for pt in &stroke.path.pts {
            encode_stroke_point(&mut buffer, &pt.prev);
            encode_stroke_point(&mut buffer, &pt.pt);
            encode_stroke_point(&mut buffer, &pt.next);
        }
        alisa::ABFValue::Binary(buffer.into_iter().collect())
    }

    fn deserialize(data: &alisa::ABFValue, _context: &mut alisa::DeserializationContext) -> Option<Self> {
        let bytes = data.as_binary()?;
        let floats: Box<[f32]> = bytes.chunks(4).filter_map(|x| {
            if x.len() != 4 {
                None
            } else {
                Some(f32::from_le_bytes([x[0], x[1], x[2], x[3]]))
            }
        }).collect();
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

    fn delete(&self, _: &mut Vec<alisa::AnyPtr>) {
        
    }

}


#[derive(Clone, alisa::Serializable)]
pub struct Stroke {
    pub frame: alisa::Ptr<Frame>,
    pub stroke: StrokeData,
    pub color: SceneObjectColor,
    pub width: f32,
    pub brush: StrokeBrush
} 

impl Default for Stroke {

    fn default() -> Self {
        Self {
            frame: alisa::Ptr::null(),
            stroke: StrokeData(malvina::Stroke::empty()),
            color: Default::default(),
            width: 5.0,
            brush: Default::default()
        }
    }

}

impl alisa::Object for Stroke {
    type Project = Project;
    const TYPE_ID: u16 = 0;

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
    pub color: SceneObjectColor,
    pub width: f32,
    pub brush: StrokeBrush
}

impl Default for StrokeTreeData {

    fn default() -> Self {
        Self {
            stroke: StrokeData(malvina::Stroke::empty()),
            color: Default::default(), 
            width: 5.0,
            brush: Default::default()
        }
    }

}

impl alisa::TreeObj for Stroke {
    type ParentPtr = alisa::Ptr<Frame>;
    type ChildList = alisa::ChildList<SceneObjPtr>;
    type TreeData = StrokeTreeData;

    fn child_list<'a>(parent: alisa::Ptr<Frame>, context: &'a alisa::ProjectContext<Project>) -> Option<&'a Self::ChildList> {
        context.obj_list().get(parent).map(|frame| &frame.scene)
    }

    fn child_list_mut<'a>(parent: alisa::Ptr<Frame>, recorder: &'a mut alisa::Recorder<Project>) -> Option<&'a mut Self::ChildList> {
        recorder.get_obj_mut(parent).map(|frame| &mut frame.scene)
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
            width: data.width,
            brush: data.brush
        };
        recorder.add_obj(ptr, stroke);
    }

    fn destroy(&self, _recorder: &mut alisa::Recorder<Project>) {

    }

    fn collect_data(&self, _objects: &Objects) -> StrokeTreeData {
        StrokeTreeData {
            stroke: self.stroke.clone(),
            color: self.color,
            width: self.width,
            brush: self.brush
        }
    }

}

alisa::tree_object_operations!(Stroke);
alisa::object_set_property_operation!(Stroke, stroke, StrokeData);
alisa::object_set_property_operation!(Stroke, color, SceneObjectColor);
