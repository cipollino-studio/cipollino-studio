use std::collections::HashMap;

use project::SceneObjPtr;

use crate::ToolContext;

use super::{Tile, ROOT_DEPTH};

pub(super) struct Segment {
    pub segment: elic::BezierSegment<elic::Vec2>,
    pub center_line_segment: elic::BezierSegment<elic::Vec2>,
    pub bounds: elic::Rect
}

fn scale_pt_about_pivot(pt: elic::Vec2, pivot: elic::Vec2, fac: f32) -> elic::Vec2 {
    pivot + fac * (pt - pivot)
}

fn add_stroke_segment(segments: &mut Vec<Segment>, segment: elic::BezierSegment<malvina::StrokePoint>, r: f32, depth: u32) {
    let pt_segment = segment.map(|pt| pt.pt);

    if depth > 0 {
        const EPS: f32 = 0.0025;
        let x_extrema = pt_segment.map(|pt| pt.x).extrema_ts().into_small_arr();
        for t in x_extrema.iter() {
            if t >= EPS && t <= 1.0 - EPS {
                let (a, b) = segment.split(t);
                add_stroke_segment(segments, a, r, depth - 1);
                add_stroke_segment(segments, b, r, depth - 1);
                return;
            } 
        }
        let y_extrema = pt_segment.map(|pt| pt.x).extrema_ts().into_small_arr();
        for t in y_extrema.iter() {
            if t >= EPS && t <= 1.0 - EPS {
                let (a, b) = segment.split(t);
                add_stroke_segment(segments, a, r, depth - 1);
                add_stroke_segment(segments, b, r, depth - 1);
                return;
            } 
        }
    }

    let l0 = elic::Line::new(pt_segment.p0, pt_segment.p0 + (pt_segment.b0 - pt_segment.p0).turn_cw());
    let l1 = elic::Line::new(pt_segment.p1, pt_segment.p1 + (pt_segment.p1 - pt_segment.a1).turn_cw());

    let mut add_segment = |segment| {
        segments.push(Segment {
            segment,
            center_line_segment: pt_segment,
            bounds: segment.bounds() 
        });
    };

    match l0.intersect(l1) {
        None => { // Normals are parallel, segment is a straight line
            let normal = pt_segment.sample_normal(0.0); 

            let p0 = pt_segment.p0 + normal * segment.p0.pressure * r; 
            let p1 = pt_segment.p1 + normal * segment.p1.pressure * r; 
            add_segment(elic::BezierSegment::straight(p0, p1));

            let p0 = pt_segment.p0 - normal * segment.p0.pressure * r; 
            let p1 = pt_segment.p1 - normal * segment.p1.pressure * r; 
            add_segment(elic::BezierSegment::straight(p0, p1));
        },
        Some(scale_pt) => {
            let dist_p0 = scale_pt.distance(pt_segment.p0);
            let dist_p1 = scale_pt.distance(pt_segment.p1);

            let offset_p0 = segment.p0.pressure * r;
            let offset_p1 = segment.p1.pressure * r;

            let cap = 5.0;

            let scale_fac_p0 = (dist_p0 + offset_p0) / dist_p0;
            let scale_fac_p1 = (dist_p1 + offset_p1) / dist_p1;
            add_segment(elic::BezierSegment {
                p0: scale_pt_about_pivot(pt_segment.p0, scale_pt, scale_fac_p0),
                b0: scale_pt_about_pivot(pt_segment.b0, scale_pt, scale_fac_p0.clamp(-cap, cap)),
                a1: scale_pt_about_pivot(pt_segment.a1, scale_pt,scale_fac_p1.clamp(-cap, cap)),
                p1: scale_pt_about_pivot(pt_segment.p1, scale_pt,scale_fac_p1)
            });

            let scale_fac_p0 = ((dist_p0 - offset_p0) / dist_p0).clamp(-cap, cap);
            let scale_fac_p1 = ((dist_p1 - offset_p1) / dist_p1).clamp(-cap, cap);
            add_segment(elic::BezierSegment {
                p0: scale_pt_about_pivot(pt_segment.p0, scale_pt, scale_fac_p0),
                b0: scale_pt_about_pivot(pt_segment.b0, scale_pt, scale_fac_p0.clamp(-cap, cap)),
                a1: scale_pt_about_pivot(pt_segment.a1, scale_pt,scale_fac_p1.clamp(-cap, cap)),
                p1: scale_pt_about_pivot(pt_segment.p1, scale_pt,scale_fac_p1)
            });
        },
    }

}

fn add_stroke_cap(segments: &mut Vec<Segment>, center: elic::Vec2, r: f32, dir: elic::Vec2) {
    let dir = dir.normalize() * r;
    let segment = elic::BezierSegment {
        p0: center + dir.turn_cw(),
        b0: center + dir.turn_cw() + dir * 1.5,
        a1: center + dir.turn_ccw() + dir * 1.5,
        p1: center + dir.turn_ccw(),
    };
    segments.push(Segment {
        segment,
        center_line_segment: elic::BezierSegment::straight(center, center),
        bounds: segment.bounds(),
    });
}

pub(super) fn get_segments(ctx: &ToolContext) -> Vec<Segment> {
    let mut segments = Vec::new();
    for obj in &ctx.render_list.objs {
        match *obj {
            SceneObjPtr::Stroke(ptr) => {
                let Some(stroke) = ctx.project.client.get(ptr) else { continue; };
                let path = &stroke.stroke.0.path;
                let r = stroke.width / 2.0;
                if path.pts.len() < 2 {
                    continue;
                }
                for segment in path.iter_segments() {
                    add_stroke_segment(&mut segments, segment, r, 5);
                }
                let pt0 = path.pts.first().unwrap();
                add_stroke_cap(&mut segments, pt0.pt.pt, r * pt0.pt.pressure, pt0.pt.pt - pt0.next.pt);
                let pt1 = path.pts.last().unwrap();
                add_stroke_cap(&mut segments, pt1.pt.pt, r * pt1.pt.pressure, pt1.pt.pt - pt1.prev.pt);
            },
            SceneObjPtr::Fill(ptr) => {
                let Some(fill) = ctx.project.client.get(ptr) else { continue; };
                for path in &fill.paths.0.paths {
                    for segment in path.iter_segments() {
                        segments.push(Segment {
                            segment,
                            center_line_segment: segment,
                            bounds: segment.bounds(),
                        });
                    }
                    if path.pts.len() > 0 {
                        let p0 = path.pts[0].pt;
                        let p1 = path.pts[path.pts.len() - 1].pt;
                        let segment = elic::BezierSegment::straight(p0, p1);
                        segments.push(Segment {
                            segment,
                            center_line_segment: segment,
                            bounds: segment.bounds() 
                        });
                    }
                }
            },
        }
    }

    segments
}

pub(super) fn calc_root_tile_contents(segments: &Vec<Segment>) -> HashMap<elic::IVec2, Vec<u32>> {

    let mut root_tile_contents = HashMap::new();
    let root_tile_size = Tile::tile_size_at_depth(ROOT_DEPTH);
    for (idx, segment) in segments.iter().enumerate() {
        let idx = idx as u32;
        let min_x = (segment.bounds.left() / root_tile_size).floor() as i32;
        let max_x = (segment.bounds.right() / root_tile_size).ceil() as i32;
        let min_y = (segment.bounds.top() / root_tile_size).floor() as i32;
        let max_y = (segment.bounds.bottom() / root_tile_size).ceil() as i32;
        for x in min_x..max_x {
            for y in min_y..max_y {
                let pt = elic::ivec2(x, y);
                if !root_tile_contents.contains_key(&pt) {
                    root_tile_contents.insert(pt, Vec::new());
                }
                root_tile_contents.get_mut(&pt).unwrap().push(idx);
            }
        }
    }

    root_tile_contents
}
