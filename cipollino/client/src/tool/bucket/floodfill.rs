
use std::collections::{HashMap, HashSet, VecDeque};

use elic::ivec2;
use pierro::wgpu;
use project::SceneObjPtr;

use crate::{bounding_boxes, EditorState, ToolContext};

use super::{calc_path, BucketTool};

const STEP_SIZE: f32 = 1.0;
const TILE_SIZE: usize = 128;
const TILE_SIZE_I32: i32 = TILE_SIZE as i32;
const TILE_SIZE_U32: u32 = TILE_SIZE as u32;
const TILE_SIZE_IVEC: elic::IVec2 = elic::IVec2::splat(TILE_SIZE_I32);
const TILE_SIZE_PX: f32 = (TILE_SIZE_I32 as f32) * STEP_SIZE;

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

struct Tile {
    xs: [[u8; TILE_SIZE]; TILE_SIZE],
    ys: [[u8; TILE_SIZE]; TILE_SIZE],
    hit: [[bool; TILE_SIZE]; TILE_SIZE],
    rect: elic::Rect
}

impl Tile {

    fn point_at(&self, x: i32, y: i32) -> Option<elic::Vec2> {
        assert!(0 <= x && x < TILE_SIZE_I32);
        assert!(0 <= y && y < TILE_SIZE_I32);
        let x = x as usize;
        let y= y as usize;
        if !self.hit[x][y] {
            return None;
        }
        let hit_x = (self.xs[x][y] as f32) / 255.0; 
        let hit_y = (self.ys[x][y] as f32) / 255.0; 
        Some(self.rect.tl() + elic::vec2(hit_x, hit_y) * self.rect.size())
    }

}

fn render_tile(editor: &mut EditorState, ctx: &mut ToolContext, tile_coord: elic::IVec2, texture: &wgpu::Texture, texture_read_buffer: &wgpu::Buffer) -> Tile {
    let rect = elic::Rect::min_size(
        elic::vec2(tile_coord.x as f32, tile_coord.y as f32) * TILE_SIZE_PX,
        elic::Vec2::splat(TILE_SIZE_PX)
    );
    let camera = malvina::Camera::new(rect.center(), 1.0 / STEP_SIZE);

    ctx.renderer.as_mut().expect("renderer should be available").render(ctx.device, ctx.queue, texture, camera, elic::Color::BLACK, 1.0, |rndr| {
        for scene_obj in ctx.render_list.objs.iter() {
            match scene_obj {
                SceneObjPtr::Stroke(stroke_ptr) => {
                    let stroke_mesh_cache = editor.stroke_mesh_cache.borrow();
                    if let Some(stroke) = stroke_mesh_cache.get(&stroke_ptr) {
                        rndr.render_stroke_bucket(stroke, elic::Color::WHITE, editor.scene_obj_transform(*stroke_ptr));
                    }
                },
                SceneObjPtr::Fill(fill_ptr) => {
                    let fill_mesh_cache = editor.fill_mesh_cache.borrow();
                    if let Some(fill) = fill_mesh_cache.get(fill_ptr) {
                        rndr.render_fill_bucket(fill, elic::Color::WHITE, editor.scene_obj_transform(*fill_ptr));
                    }
                }
            }
        }
    });

    let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("cipollino_fill_texture_copy_encoder"),
    });
    let texture_copy_source = wgpu::ImageCopyTextureBase {
        texture,
        mip_level: 0,
        origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
        aspect: wgpu::TextureAspect::All,
    };
    let texture_copy_dest = wgpu::ImageCopyBufferBase {
        buffer: texture_read_buffer,
        layout: wgpu::ImageDataLayout { offset: 0, bytes_per_row: Some(4 * TILE_SIZE_U32), rows_per_image: None },
    };
    encoder.copy_texture_to_buffer(texture_copy_source, texture_copy_dest, wgpu::Extent3d { width: TILE_SIZE_U32, height: TILE_SIZE_U32, depth_or_array_layers: 1 });
    ctx.queue.submit([encoder.finish()]);


    texture_read_buffer.slice(..).map_async(wgpu::MapMode::Read, |_| {});
    ctx.device.poll(wgpu::MaintainBase::Wait);

    let pixel_data = texture_read_buffer.slice(..).get_mapped_range();
    let mut tile = Tile {
        xs: [[0; TILE_SIZE]; TILE_SIZE],
        ys: [[0; TILE_SIZE]; TILE_SIZE],
        hit: [[false; TILE_SIZE]; TILE_SIZE],
        rect,
    };

    for x in 0..TILE_SIZE {
        for y in 0..TILE_SIZE {
            let idx = (x + y * TILE_SIZE) * 4;
            tile.xs[x][TILE_SIZE - 1 - y] = pixel_data[idx + 0];
            tile.ys[x][TILE_SIZE - 1 - y] = pixel_data[idx + 1];
            tile.hit[x][TILE_SIZE - 1 - y] = pixel_data[idx + 2] != 0;
        }
    }

    drop(pixel_data);
    texture_read_buffer.unmap();

    tile
}

fn count_neighbours(hits: &HashMap<elic::IVec2, elic::Vec2>, pt: elic::IVec2) -> i32 {
    let mut n_neighbours = 0;
    for dir in DIRS8 {
        if hits.contains_key(&(pt + *dir)) {
            n_neighbours += 1;
        }
    }
    n_neighbours
}

fn cleanup_boundary(hits: &mut HashMap<elic::IVec2, elic::Vec2>) {
    
    // Remove tails
    let mut bfs = VecDeque::new();
    for pt in hits.keys() {
        if count_neighbours(hits, *pt) == 1 {
            bfs.push_back(*pt);
        }
    }

    while let Some(pt) = bfs.pop_front() {
        hits.remove(&pt); 
        for dir in DIRS8 {
            let next = pt + *dir;
            if hits.contains_key(&next) && count_neighbours(hits, next) == 1 {
                bfs.push_back(next);
            } 
        }
    }

    // Remove corners
    let mut corners = Vec::new();
    for pt in hits.keys() {
        let pt = *pt;
        let up = hits.contains_key(&(pt + elic::IVec2::Y));
        let right = hits.contains_key(&(pt + elic::IVec2::X));
        let down = hits.contains_key(&(pt - elic::IVec2::Y));
        let left = hits.contains_key(&(pt - elic::IVec2::X));

        let horizontal = left || right;
        let vertical = up || down;

        if horizontal && vertical && count_neighbours(hits, pt) == 2 {
            corners.push(pt);
        }
    }
    for corner in corners {
        hits.remove(&corner);
    }

}

pub(super) fn floodfill(editor: &mut EditorState, ctx: &mut ToolContext, mouse_pos: elic::Vec2) {

    let texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("cipollino_fill_texture"),
        size: wgpu::Extent3d { width: TILE_SIZE_U32, height: TILE_SIZE_U32, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });

    let texture_read_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("cipollino_fill_read_buffer"),
        size: (TILE_SIZE_U32 * TILE_SIZE_U32 * 4) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    if let Some((x, y)) = ctx.picking_mouse_pos {
        match ctx.pick(x, y) {
            Some(_) => {
                return;
            },
            _ => {}
        }
    }

    let mut scene_bounds: Option<elic::Rect> = None;
    for scene_obj in ctx.render_list.objs.iter() {
        match scene_obj {
            SceneObjPtr::Stroke(ptr) => {
                let Some(stroke) = ctx.project.client.get(*ptr) else { continue; };
                let Some(stroke_bounds) = bounding_boxes::stroke(stroke) else { continue; };
                scene_bounds = Some(scene_bounds.map(|bounds| bounds.merge(stroke_bounds)).unwrap_or(stroke_bounds));
            },
            SceneObjPtr::Fill(ptr) => {
                let Some(fill) = ctx.project.client.get(*ptr) else { continue; };
                let Some(fill_bounds) = bounding_boxes::fill(fill) else { continue; };
                scene_bounds = Some(scene_bounds.map(|bounds| bounds.merge(fill_bounds)).unwrap_or(fill_bounds));
            },
        }
    }
    let Some(scene_bounds) = scene_bounds else { return; };
    // Expand the scene bounding box a bit just in case
    let scene_bounds = elic::Rect::min_max(
        scene_bounds.tl() - elic::Vec2::ONE * STEP_SIZE * 2.0,
        scene_bounds.br() + elic::Vec2::ONE * STEP_SIZE * 2.0
    ); 

    let mouse_pos = ivec2((mouse_pos.x / STEP_SIZE).round() as i32, (mouse_pos.y / STEP_SIZE).round() as i32);
    let mut vis = HashSet::new();
    let mut tiles = HashMap::new();
    let mut bfs = VecDeque::new();
    let mut hits = HashMap::new();
    bfs.push_back(mouse_pos);
    while let Some(pos) = bfs.pop_front() {
        if vis.contains(&pos) {
            continue;
        }
        vis.insert(pos);

        // The floodfill leaks, no point continuing
        if !scene_bounds.contains(elic::vec2(pos.x as f32, pos.y as f32) * STEP_SIZE) {
            return;
        }

        let tile_coord = pos.div_euclid(TILE_SIZE_IVEC);
        let in_tile_coord = pos - tile_coord * TILE_SIZE_IVEC; 
        if !tiles.contains_key(&tile_coord) {
            let tile = render_tile(editor, ctx, tile_coord, &texture, &texture_read_buffer);
            tiles.insert(tile_coord, tile);
        }
        let tile = tiles.get(&tile_coord).expect("we just checked to make sure the tile exists...");
        
        if let Some(pt) = tile.point_at(in_tile_coord.x, in_tile_coord.y) {
            hits.insert(pos, pt); 
            continue;
        }

        for dir in elic::IVec2::CARDINAL_DIRECTIONS {
            bfs.push_back(pos + dir);
        }
    }

    cleanup_boundary(&mut hits);

    let mut paths = Vec::new();
    while let Some(first) = hits.keys().min_by(|a, b| if a.x == b.x { a.y.cmp(&b.y) } else { a.x.cmp(&b.x) }) {
        let first = *first;
        let mut path_pts = vec![
            hits.remove(&first).expect("first must be a key")
        ];

        let mut add_pt = |pt| {
            if path_pts.last().unwrap().distance(pt) > 0.5 {
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

        paths.push(calc_path(&path_pts));

        cleanup_boundary(&mut hits);
    }

    BucketTool::create_fill(editor, ctx, malvina::FillPaths { paths });
}
