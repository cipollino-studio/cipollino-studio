
use std::collections::HashMap;
use project::{Client, Fill, Ptr, SceneObjPtr, Stroke};

use crate::bounding_boxes;

use super::SceneRenderList;

pub struct StrokeMesh {
    pub mesh: malvina::StrokeMesh,
    pub bounds: Option<elic::Rect>,
    pub color: elic::Color
}

pub struct FillMesh {
    pub mesh: malvina::FillMesh,
    pub bounds: Option<elic::Rect>,
    pub color: elic::Color
}

pub struct MeshCache {
    strokes: HashMap<Ptr<Stroke>, StrokeMesh>,
    fills: HashMap<Ptr<Fill>, FillMesh>
}

impl MeshCache {

    pub fn new() -> Self {
        Self {
            strokes: HashMap::new(),
            fills: HashMap::new(),
        }
    }

    fn calculate_stroke_mesh(&mut self, stroke_ptr: Ptr<Stroke>, client: &Client, device: &pierro::wgpu::Device) {
        if self.strokes.contains_key(&stroke_ptr) {
            return;
        }

        let Some(stroke) = client.get(stroke_ptr) else { return; };
        let mesh = malvina::StrokeMesh::new(device, &stroke.stroke.0, stroke.width); 

        self.strokes.insert(stroke_ptr, StrokeMesh {
            mesh,
            bounds: bounding_boxes::stroke(stroke),
            color: stroke.color.into()
        });
    }

    fn calculate_fill_mesh(&mut self, fill_ptr: Ptr<Fill>, client: &Client, device: &pierro::wgpu::Device) {
        if self.fills.contains_key(&fill_ptr) {
            return;
        }

        let Some(fill) = client.get(fill_ptr) else { return; };
        let mesh = malvina::FillMesh::new(device, &fill.paths.0);

        self.fills.insert(fill_ptr, FillMesh {
            mesh,
            bounds: bounding_boxes::fill(fill),
            color: fill.color.into()
        });
    }

    pub fn calculate(&mut self, render_list: &SceneRenderList, client: &Client, device: &pierro::wgpu::Device) {
        for obj in &render_list.objs {
            match *obj {
                SceneObjPtr::Stroke(stroke) => {
                    self.calculate_stroke_mesh(stroke, client, device);
                },
                SceneObjPtr::Fill(fill) => {
                    self.calculate_fill_mesh(fill, client, device);
                },
            } 
        }
    }

    pub fn invalidate(&mut self, client: &Client) {
        for updated_stroke in client.modified() {
            self.strokes.remove(&updated_stroke);
        }
        for updated_fill in client.modified() {
            self.fills.remove(&updated_fill);
        }
    }

    pub fn get_stroke(&self, stroke_ptr: Ptr<Stroke>) -> Option<&StrokeMesh> {
        self.strokes.get(&stroke_ptr)
    }

    pub fn get_fill(&self, fill_ptr: Ptr<Fill>) -> Option<&FillMesh> {
        self.fills.get(&fill_ptr)
    }

}
