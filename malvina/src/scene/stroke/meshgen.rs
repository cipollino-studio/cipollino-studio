
use crate::{sample, sample_derivative, HasMagnitude, StrokeStampInstance};

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

        let spacing = 1.0;

        for i in 0..(self.path.pts.len() - 1) {
            let mut t = 0.0;
            while t < 1.0 {
                let a = &self.path.pts[i];
                let b = &self.path.pts[i + 1];

                let bezier_pt = sample(a, b, t);
                let pos = bezier_pt.pt;
                let pressure = bezier_pt.pressure;
                let derivative = sample_derivative(a, b, t).pt;
                let tangent = derivative.normalize();
                let right = tangent * 3.0 * pressure;
                stamps.push(StrokeStampInstance {
                    pos, 
                    right
                });

                t += spacing / derivative.magnitude(); 
            }
        }

        stamps
    }

}
