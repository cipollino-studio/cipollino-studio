
use crate::FillVertex;

use super::FillPaths;

fn meshgen_path(path: &elic::BezierPath<elic::Vec2>, verts: &mut Vec<FillVertex>) {
    if path.pts.len() == 0 {
        return;
    }

    let mut pts = Vec::new();

    let mut t = 0.0;
    let mut prev_t = 0.0;
    let pt_0 = path.sample(0.0);
    let mut prev_pt = pt_0;
    pts.push(pt_0);

    while t < (path.pts.len() - 1) as f32 {
        t += 0.0025;
        let pt = path.sample(t);
        let distance = pt.distance(prev_pt);
        let target_distance = 1.0;
        if distance >= target_distance {
            let scale_fac = target_distance / distance;
            t = (prev_t + scale_fac * (t - prev_t)).max(prev_t + 0.0005);
            let pt = path.sample(t);
            pts.push(pt);

            prev_t = t;
            prev_pt = pt;
        }
    }
    pts.push(path.sample((path.pts.len() - 1) as f32));

    // It doesn't matter where we put the triangle fan center, but putting it in the rough middle
    // will hopefully help avoid some overdraw
    let center = elic::Rect::bounds_all(pts.iter().copied()).center();
    for i in 0..pts.len() {
        let a = pts[i];
        let b = if i == pts.len() - 1 { pts[0] } else { pts[i + 1] };
        verts.push(FillVertex { pos: center.into() });
        verts.push(FillVertex { pos: a.into() });
        verts.push(FillVertex { pos: b.into() });
    }
}

impl FillPaths {

    pub(crate) fn meshgen(&self) -> Vec<FillVertex> {
        let mut verts = Vec::new();
        for path in &self.paths {
            meshgen_path(path, &mut verts);
        }
        verts
    }

}
