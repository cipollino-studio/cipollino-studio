
use crate::StrokeStampInstance;

use super::Stroke;

impl Stroke {

    pub(crate) fn meshgen(&self, width: f32) -> Vec<StrokeStampInstance> {
        let radius = width / 2.0;

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

        let mut t = 0.0;
        let mut prev_t = 0.0;
        let pt_0 = self.path.sample(0.0);
        let tang_0 = self.path.sample_derivative(0.0).pt.normalize();
        let mut prev_pt = pt_0.pt;
        let mut prev_size = pt_0.pressure * radius;
        stamps.push(StrokeStampInstance {
            pos: prev_pt.into(),
            right: (tang_0 * prev_size).into(),
        });

        while t < (self.path.pts.len() - 1) as f32 {
            t += 0.0025;
            let pt = self.path.sample(t);
            let size = pt.pressure * radius;
            let distance = pt.pt.distance(prev_pt);
            let target_distance = (prev_size + size) * 0.08;
            if distance >= target_distance {
                let scale_fac = target_distance / distance;
                t = (prev_t + scale_fac * (t - prev_t)).max(prev_t + 0.0005);
                let pt = self.path.sample(t);
                let size = pt.pressure * radius;
                let tang = self.path.sample_derivative(t).pt.normalize();
                stamps.push(StrokeStampInstance {
                    pos: pt.pt.into(),
                    right: (tang * size).into(),
                });
                prev_t = t;
                prev_pt = pt.pt;
                prev_size = size;
            }
        }

        stamps
    }

}
