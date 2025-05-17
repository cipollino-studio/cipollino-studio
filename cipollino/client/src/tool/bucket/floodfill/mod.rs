
use crate::{EditorState, ToolContext};

use super::BucketTool;

mod segments;
use segments::*;

mod quadtree;
use quadtree::*;

mod bfs;
use bfs::*;

mod boundary_cleanup;
use boundary_cleanup::*;

mod paths;
use paths::*;

pub(super) fn floodfill(editor: &mut EditorState, ctx: &mut ToolContext, click_pt: elic::Vec2) {
    let segments = get_segments(&ctx);
    if segments.len() == 0 {
        return;
    }

    let root_tile_contents = calc_root_tile_contents(&segments);

    let mut bounds = segments[0].bounds;
    for segment in &segments[1..] {
        bounds = bounds.merge(segment.bounds);
    }

    let Some(hits) = bfs(click_pt, &segments, root_tile_contents, bounds) else { return; };

    let paths = calc_paths(hits);

    BucketTool::create_fill(editor, ctx, malvina::FillPaths { paths });
}

pub(super) fn overlay_collision_segments(ctx: &mut ToolContext, rndr: &mut malvina::LayerRenderer) {
    let segments = get_segments(&ctx);

    for segment in segments {
        for i in 0..20 {
            let t0 = (i as f32) / 20.0;
            let t1 = (i as f32 + 1.0) / 20.0;
            rndr.overlay_line(segment.segment.sample(t0), segment.segment.sample(t1), elic::Color::RED);
        }
    }
}