
use crate::StrokeStampInstance;

use super::Stroke;

impl Stroke {

    pub(crate) fn meshgen(&self) -> Vec<StrokeStampInstance> {

        if self.path.pts.is_empty() {
            return Vec::new();
        }
        if self.path.pts.len() == 1 {
            return vec![
                StrokeStampInstance {
                    pos: self.path.pts[0].pt.pt.into(),
                    right: (elic::Vec2::X * 3.0).into()
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
                let segment = elic::BezierSegment::from_points(*a, *b);

                let bezier_pt = segment.sample(t); 
                let pos = bezier_pt.pt;
                let pressure = bezier_pt.pressure;
                let derivative = segment.sample_derivative(t).pt;
                let tangent = derivative.normalize();
                let right = tangent * 3.0 * pressure;
                stamps.push(StrokeStampInstance {
                    pos: pos.into(),
                    right: right.into()
                });

                t += spacing / derivative.length(); 
            }
        }

        stamps
    }

}
