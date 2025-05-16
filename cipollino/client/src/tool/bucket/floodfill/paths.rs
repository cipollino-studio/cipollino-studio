
use std::collections::HashMap;

use crate::tool::bucket::calc_path;

use super::cleanup_boundary;

const DIRS8: &[elic::IVec2] = &[
    elic::ivec2(0, 1),
    elic::ivec2(-1, 1),
    elic::ivec2(-1, 0),
    elic::ivec2(-1, -1),
    elic::ivec2(0, -1),
    elic::ivec2(1, -1),
    elic::ivec2(1, 0),
    elic::ivec2(1, 1),
];

pub(super) fn calc_paths(mut hits: HashMap<elic::IVec2, elic::Vec2>) -> Vec<elic::BezierPath<elic::Vec2>> {
    cleanup_boundary(&mut hits);

    let mut paths = Vec::new();
    while let Some(first) = hits.keys().min_by(|a, b| if a.x == b.x { a.y.cmp(&b.y) } else { a.x.cmp(&b.x) }) {
        let first = *first;
        let mut path_pts = vec![
            hits.remove(&first).expect("first must be a key")
        ];

        let mut add_pt = |pt| {
            if path_pts.last().unwrap().distance(pt) > 0.05 {
                path_pts.push(pt);
            }
        };

        let mut pt = first;
        let mut curr_dir = 0;
        loop {
            let mut found = false;
            for _ in 0..DIRS8.len() {
                let next = pt + DIRS8[curr_dir];
                if let Some(path_pt) = hits.remove(&next) {
                    add_pt(path_pt);
                    found = true;
                    pt = next;
                    break;
                }
                curr_dir += 1;
                curr_dir %= DIRS8.len();
            }
            if !found {
                break;
            }
        }

        if path_pts.len() < 2 {
            continue;
        }

        path_pts.push(path_pts[0]);

        paths.push(calc_path(&path_pts, 0.1));

        cleanup_boundary(&mut hits);
    }

    paths
}
