use std::collections::{HashMap, HashSet};

use super::Segment;


pub(super) const CELL_SIZE: f32 = 0.2;
pub(super) const ROOT_DEPTH: u32 = 8;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Side {
    Top,
    Right,
    Bottom,
    Left
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub(super) struct Tile {
    pub pt: elic::IVec2,
    pub depth: u32
}

impl Tile {

    fn children(&self) -> [Tile; 4] {
        [
            Tile { pt: 2 * self.pt + elic::ivec2(0, 0), depth: self.depth - 1 },
            Tile { pt: 2 * self.pt + elic::ivec2(1, 0), depth: self.depth - 1 },
            Tile { pt: 2 * self.pt + elic::ivec2(0, 1), depth: self.depth - 1 },
            Tile { pt: 2 * self.pt + elic::ivec2(1, 1), depth: self.depth - 1 },
        ]
    }

    pub fn children_on_side(&self, side: Side) -> [Tile; 2] {
        match side {
            Side::Top => [
                Tile { pt: 2 * self.pt + elic::ivec2(0, 1), depth: self.depth - 1 },
                Tile { pt: 2 * self.pt + elic::ivec2(1, 1), depth: self.depth - 1 },
            ],
            Side::Right => [
                Tile { pt: 2 * self.pt + elic::ivec2(1, 0), depth: self.depth - 1 },
                Tile { pt: 2 * self.pt + elic::ivec2(1, 1), depth: self.depth - 1 },
            ],
            Side::Bottom => [
                Tile { pt: 2 * self.pt + elic::ivec2(0, 0), depth: self.depth - 1 },
                Tile { pt: 2 * self.pt + elic::ivec2(1, 0), depth: self.depth - 1 },
            ],
            Side::Left => [
                Tile { pt: 2 * self.pt + elic::ivec2(0, 0), depth: self.depth - 1 },
                Tile { pt: 2 * self.pt + elic::ivec2(0, 1), depth: self.depth - 1 },
            ],
        }
    }

    pub fn parent(&self) -> Tile {
        Self {
            pt: self.pt.div_euclid(elic::IVec2::splat(2)),
            depth: self.depth + 1,
        }
    }

    pub fn up(&self) -> Tile {
        Tile {
            pt: self.pt + elic::IVec2::Y,
            depth: self.depth,
        }
    }

    pub fn right(&self) -> Tile {
        Tile {
            pt: self.pt + elic::IVec2::X,
            depth: self.depth,
        }
    }

    pub fn down(&self) -> Tile {
        Tile {
            pt: self.pt - elic::IVec2::Y,
            depth: self.depth,
        }
    }

    pub fn left(&self) -> Tile {
        Tile {
            pt: self.pt - elic::IVec2::X,
            depth: self.depth,
        }
    }

    pub fn root_tile(&self) -> Tile {
        Self {
            pt: elic::ivec2(
                self.pt.x >> (ROOT_DEPTH - self.depth),
                self.pt.y >> (ROOT_DEPTH - self.depth),
            ),
            depth: ROOT_DEPTH,
        }
    }

    pub fn tile_size_at_depth(depth: u32) -> f32 {
        ((1 << depth) as f32) * CELL_SIZE
    }

    pub fn rect_of(tile: Tile) -> elic::Rect {
        let tile_size = Self::tile_size_at_depth(tile.depth); 
        let tl = tile.pt.to_vec() * tile_size; 
        elic::Rect::min_size(tl, elic::Vec2::splat(tile_size))
    }

    pub fn tile_at(pt: elic::Vec2, depth: u32) -> Tile {
        let tile_size = Self::tile_size_at_depth(depth); 
        let x = (pt.x / tile_size).floor() as i32;
        let y = (pt.y / tile_size).floor() as i32;
        Tile {
            pt: elic::ivec2(x, y),
            depth,
        }
    }

}

pub(super) struct QuadTree {
    occupied: HashSet<Tile>,
    hits: HashMap<elic::IVec2, elic::Vec2>
}

impl QuadTree {

    pub fn new() -> Self {
        Self {
            occupied: HashSet::new(),
            hits: HashMap::new()
        }
    }

    pub fn occupied(&self, tile: Tile) -> bool {
        self.occupied.contains(&tile)
    }

    pub fn get_hit_pt(&self, pt: elic::IVec2) -> Option<elic::Vec2> {
        self.hits.get(&pt).copied()
    }

    fn contains_segment(segment: &Segment, tile: Tile) -> bool {
        let rect = Tile::rect_of(tile);
        if !rect.intersects(segment.bounds) {
            return false;
        }
        let segment = &segment.segment;
        if rect.contains(segment.p0) || rect.contains(segment.p1) { 
            return true;
        }
        if !rect.left_side().intersect_bezier_ts(&segment).is_empty() {
            return true;
        }
        if !rect.top_side().intersect_bezier_ts(&segment).is_empty() {
            return true;
        }
        if !rect.right_side().intersect_bezier_ts(&segment).is_empty() {
            return true;
        }
        if !rect.bottom_side().intersect_bezier_ts(&segment).is_empty() {
            return true;
        }
        false
    }

    pub fn add_segment(&mut self, segment: &Segment, tile: Tile) {
        if !Self::contains_segment(segment, tile) {
            return;
        }
        self.occupied.insert(tile);
        if tile.depth == 0 {
            let rect = Tile::rect_of(tile);
            let hit = 'hit: {
                if rect.contains(segment.segment.p0) {
                    break 'hit segment.segment.p0;
                }
                if rect.contains(segment.segment.p1) {
                    break 'hit segment.segment.p1;
                }
                let left_ts = rect.left_side().intersect_bezier_ts(&segment.segment);
                if left_ts.len() > 0 {
                    break 'hit segment.segment.sample(left_ts[0]);
                }
                let top_ts = rect.top_side().intersect_bezier_ts(&segment.segment);
                if top_ts.len() > 0 {
                    break 'hit segment.segment.sample(top_ts[0]);
                }
                let right_ts = rect.right_side().intersect_bezier_ts(&segment.segment);
                if right_ts.len() > 0 {
                    break 'hit segment.segment.sample(right_ts[0]);
                }
                let bottom_ts = rect.bottom_side().intersect_bezier_ts(&segment.segment);
                if bottom_ts.len() > 0 {
                    break 'hit segment.segment.sample(bottom_ts[0]);
                }
                rect.center()
            };
            self.hits.insert(tile.pt, hit);
            return;
        }
        for child in tile.children() {
            self.add_segment(segment, child);
        }
    }

}

