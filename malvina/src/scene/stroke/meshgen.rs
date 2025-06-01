
use rand::{Rng, SeedableRng};

use crate::{BrushSettings, StrokeStampInstance};

use super::Stroke;

impl Stroke {

    pub(crate) fn meshgen(&self, width: f32, brush: &BrushSettings) -> Vec<StrokeStampInstance> {
        let radius = width / 2.0;

        if self.path.pts.is_empty() {
            return Vec::new();
        }
        if self.path.pts.len() == 1 {
            return vec![
                StrokeStampInstance {
                    pos: self.path.pts[0].pt.pt.into(),
                    right: (elic::Vec2::X * radius).into()
                }
            ];
        }

        let mut stamps = Vec::new();
        
        let mut rng = rand::rngs::SmallRng::from_seed([0; 32]);

        let mut angle_rng = rand::rngs::SmallRng::from_seed([5; 32]);
        let mut stamp_angle = || {
            let deflection = if brush.angle_range > 0.0 {
                angle_rng.random_range(-brush.angle_range..brush.angle_range)
            } else {
                0.0
            };
            let angle_deg = brush.base_angle + deflection;
            angle_deg.to_radians()
        };

        let mut t = 0.0;
        let mut prev_t = 0.0;
        let pt_0 = self.path.sample(0.0);
        let tang_0 = self.path.sample_derivative(0.0).pt.normalize();
        let mut prev_pt = pt_0.pt;
        let mut prev_size = pt_0.pressure * radius;
        stamps.push(StrokeStampInstance {
            pos: prev_pt.into(),
            right: (tang_0 * prev_size).rotate(stamp_angle()).into(),
        });
        while t < (self.path.pts.len() - 1) as f32 {
            t += 0.0025;
            let pt = self.path.sample(t);
            let size = pt.pressure * radius;
            let distance = pt.pt.distance(prev_pt);
            let target_distance = (prev_size + size) * 0.08 * brush.stamp_spacing;
            if distance >= target_distance {

                let scale_fac = target_distance / distance;
                t = (prev_t + scale_fac * (t - prev_t)).max(prev_t + 0.0005);
                let pt = self.path.sample(t);
                let size = pt.pressure * radius;
                let tang = self.path.sample_derivative(t).pt.normalize();

                let norm = tang.turn_ccw();
                let shift = if brush.shift_range > 0.0 {
                    rng.random_range(-brush.shift_range..brush.shift_range)
                } else {
                    0.0
                };
                
                stamps.push(StrokeStampInstance {
                    pos: (pt.pt + norm * size * shift).into(),
                    right: (tang * size).rotate(stamp_angle()).into(),
                });

                prev_t = t;
                prev_pt = pt.pt;
                prev_size = size;
            }
        }

        stamps
    }

}
