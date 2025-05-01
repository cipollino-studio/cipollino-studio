
mod meshgen;

#[derive(Clone, Default)]
pub struct FillPaths {
    pub paths: Vec<elic::BezierPath<elic::Vec2>>
}
