
mod cell_rules;
use cell_rules::CELL_RULES;

use std::collections::{HashMap, VecDeque};

fn redundant(hits: &HashMap<elic::IVec2, elic::Vec2>, pt: elic::IVec2) -> bool {
    let lu : usize = hits.contains_key(&(pt + elic::ivec2(-1,  1))).into();
    let u  : usize = hits.contains_key(&(pt + elic::ivec2( 0,  1))).into();
    let ru : usize = hits.contains_key(&(pt + elic::ivec2( 1,  1))).into();
    let l  : usize = hits.contains_key(&(pt + elic::ivec2(-1,  0))).into();
    let r  : usize = hits.contains_key(&(pt + elic::ivec2( 1,  0))).into();
    let ld : usize = hits.contains_key(&(pt + elic::ivec2(-1, -1))).into();
    let d  : usize = hits.contains_key(&(pt + elic::ivec2( 0, -1))).into();
    let rd : usize = hits.contains_key(&(pt + elic::ivec2( 1, -1))).into();
    let rule_idx =
        (lu << 0) |
        (u << 1) |
        (ru << 2) |
        (l << 3) |
        (r << 4) |
        (ld << 5) |
        (d << 6) |
        (rd << 7);
    !CELL_RULES[rule_idx]
}

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

pub(super) fn cleanup_boundary(hits: &mut HashMap<elic::IVec2, elic::Vec2>) {
    
    let mut bfs = VecDeque::new();
    for pt in hits.keys() {
        if redundant(hits, *pt) {
            bfs.push_back(*pt);
        }
    }

    while let Some(pt) = bfs.pop_front() {
        hits.remove(&pt); 
        for dir in DIRS8 {
            let next = pt + *dir;
            if hits.contains_key(&next) && redundant(hits, next) {
                bfs.push_back(next);
            } 
        }
    }

}
