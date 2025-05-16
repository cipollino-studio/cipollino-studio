use std::collections::HashMap;

use project::SceneObjPtr;

use crate::ToolContext;

use super::{Tile, ROOT_DEPTH};


pub(super) struct Segment {
    pub segment: elic::BezierSegment<elic::Vec2>,
    pub bounds: elic::Rect
}

pub(super) fn get_segments(ctx: &ToolContext) -> Vec<Segment> {
    let mut segments = Vec::new();
    for obj in &ctx.render_list.objs {
        match *obj {
            SceneObjPtr::Stroke(ptr) => {
                let Some(stroke) = ctx.project.client.get(ptr) else { continue; };
                for segment in stroke.stroke.0.path.iter_segments() {
                    let segment = segment.map(|pt| pt.pt);
                    segments.push(Segment {
                        segment,
                        bounds: segment.bounds(),
                    });
                }
            },
            SceneObjPtr::Fill(ptr) => {
                let Some(fill) = ctx.project.client.get(ptr) else { continue; };
                for path in &fill.paths.0.paths {
                    for segment in path.iter_segments() {
                        segments.push(Segment {
                            segment,
                            bounds: segment.bounds(),
                        });
                    }
                    if path.pts.len() > 0 {
                        let p0 = path.pts[0].pt;
                        let p1 = path.pts[path.pts.len() - 1].pt;
                        let segment = elic::BezierSegment {
                            p0,
                            b0: (2.0 * p0 + p1) / 3.0,
                            a1: (p0 + 2.0 * p1) / 3.0,
                            p1,
                        };
                        segments.push(Segment {
                            segment: segment,
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
