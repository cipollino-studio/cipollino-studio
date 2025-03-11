
#[derive(Clone, Copy)]
pub struct Camera {
    center: glam::Vec2,
    zoom: f32
}

impl Camera {

    pub fn new(center_x: f32, center_y: f32, zoom: f32) -> Self {
        Camera {
            center: glam::vec2(center_x, center_y),
            zoom: zoom,
        }
    }

    pub(crate) fn calc_view_proj(&self, resolution: glam::Vec2) -> glam::Mat4 {
        let min = self.center - resolution * 0.5 / self.zoom;
        let max = self.center + resolution * 0.5 / self.zoom;
        glam::Mat4::orthographic_rh(min.x, max.x, min.y, max.y, -1.0, 1.0)
    }

    pub fn screen_to_world(&self, pos: glam::Vec2, resolution: glam::Vec2) -> glam::Vec2 {
        let view_proj = self.calc_view_proj(resolution);
        let ndc = (pos - resolution * 0.5) / (resolution * 0.5);
        let ndc = glam::vec4(ndc.x, ndc.y, 0.0, 1.0);
        let world = view_proj.inverse() * ndc;
        glam::vec2(world.x, world.y) 
    }

}
