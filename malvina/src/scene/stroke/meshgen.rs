
use crate::{HasMagnitude, StrokeStampInstance};

use super::Stroke;

impl Stroke {

    pub(crate) fn meshgen(&self) -> Vec<StrokeStampInstance> {

        if self.path.pts.is_empty() {
            return Vec::new();
        }
        if self.path.pts.len() == 1 {
            return vec![
                StrokeStampInstance {
                    pos: self.path.pts[0].pt.pt,
                    right: glam::Vec2::X * 3.0
                }
            ];
        }

        let mut stamps = Vec::new();

        let mut t = 0.0;
        while t < 2.0 {
            let pos = self.path.sample(t).pt;
            let derivative = self.path.sample_derivative(t).pt;
            let tangent = derivative.normalize();
            let right = tangent * 3.0;
            stamps.push(StrokeStampInstance {
                pos: pos + glam::Vec2::Y,
                right
            });

            t += 30.0 / derivative.magnitude(); 
        }

        stamps
    }

}
