use std::collections::{HashMap, HashSet, VecDeque};

use super::{QuadTree, Segment, Side, Tile, ROOT_DEPTH};


pub(super) fn bfs(
    click_pt: elic::Vec2,
    segments: &Vec<Segment>,
    mut root_tile_contents: HashMap<elic::IVec2, Vec<u32>>,
    bounds: elic::Rect
) -> Option<HashMap<elic::IVec2, elic::Vec2>> {
    let mut tree = QuadTree::new();
    let mut root_tiles_explored = HashSet::new();
    let init_root_tile = Tile::tile_at(click_pt, ROOT_DEPTH);
    root_tiles_explored.insert(init_root_tile);
    if let Some(segment_idxs) = root_tile_contents.remove(&init_root_tile.pt) {
        for segment_idx in segment_idxs {
            let segment = &segments[segment_idx as usize];
            tree.add_segment(segment, init_root_tile);
        }
    }

    let mut init_depth = ROOT_DEPTH;
    while tree.occupied(Tile::tile_at(click_pt, init_depth)) {
        if init_depth == 0 {
            return None;
        }
        init_depth -= 1;
    }
    let init_tile = Tile::tile_at(click_pt, init_depth);

    // Step 5: BFS
    let mut vis = HashSet::new();
    let mut bfs = VecDeque::new();
    bfs.push_back((init_tile, Side::Top));
    let mut hits = HashMap::new();

    while let Some((mut curr, origin)) = bfs.pop_front() {
        if vis.contains(&curr) {
            continue;
        }

        let rect = Tile::rect_of(curr);
        if !rect.intersects(bounds) {
            // We leak outside the scene, no point continuing
            return None;
        }

        let root_tile = curr.root_tile();
        if let Some(segment_idxs) = root_tile_contents.remove(&root_tile.pt) {
            for segment_idx in segment_idxs {
                let segment = &segments[segment_idx as usize];
                tree.add_segment(segment, root_tile);
            }
        }

        // Subdivision
        if tree.occupied(curr) {
            if curr.depth == 0 {
                let hit_pt = curr.pt;
                hits.insert(hit_pt, tree.get_hit_pt(curr.pt).unwrap_or(rect.center()));
                continue;
            }
            for child in curr.children_on_side(origin) {
                bfs.push_back((child, origin));
            }
            continue;
        }

        // Growth
        let mut parent = curr.parent();
        while parent.depth <= ROOT_DEPTH && !tree.occupied(parent) && !vis.contains(&parent) {
            curr = parent;
            parent = curr.parent();
        }
        if vis.contains(&parent) || vis.contains(&curr) {
            continue;
        }
        vis.insert(curr);

        // Exploration
        bfs.push_back((curr.up(), Side::Bottom));
        bfs.push_back((curr.right(), Side::Left));
        bfs.push_back((curr.down(), Side::Top));
        bfs.push_back((curr.left(), Side::Right));
    }

    Some(hits)
}