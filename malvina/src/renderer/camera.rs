
#[derive(Clone, Copy)]
pub struct Camera {
    pub(crate) center: elic::Vec2,
    pub(crate) zoom: f32,
    pub(crate) mirror: bool,
}

impl Camera {

    pub fn new(center: elic::Vec2, zoom: f32) -> Self {
        Camera {
            center,
            zoom: zoom,
            mirror: false
        }
    }

    pub fn mirror(mut self, mirror: bool) -> Self {
        self.mirror = mirror;
        self
    }

    pub fn calc_view(&self) -> elic::Mat4 {
        elic::Mat4::scale(if self.mirror {
            elic::vec2(-1.0, 1.0)
        } else {
            elic::vec2( 1.0, 1.0)
        })
    }

    pub fn calc_proj(&self, resolution: elic::Vec2) -> elic::Mat4 {
        let min = self.center - resolution * 0.5 / self.zoom;
        let max = self.center + resolution * 0.5 / self.zoom;
        elic::Mat4::orthographic(min.x, max.x, min.y, max.y)
    }

    pub fn calc_view_proj(&self, resolution: elic::Vec2) -> elic::Mat4 {
        let view = self.calc_view();
        let proj = self.calc_proj(resolution);
        view * proj
    }

    pub fn screen_to_world(&self, pos: elic::Vec2, resolution: elic::Vec2) -> elic::Vec2 {
        let view_proj = self.calc_view_proj(resolution);
        let ndc = (pos - resolution * 0.5) / (resolution * 0.5);
        let ndc = elic::vec4(ndc.x, ndc.y, 0.0, 1.0);
        let world = view_proj.inverse() * ndc;
        elic::vec2(world.x, world.y) 
    }

}
