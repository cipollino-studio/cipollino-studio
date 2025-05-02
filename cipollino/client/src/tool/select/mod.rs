
use gizmos::PotentialDragState;
use project::{Action, Client, Fill, FillPaths, SetFillPaths, SetStrokeStroke, Stroke, StrokeData};
use crate::{keyboard_shortcut, EditorState, Selection};

use super::{bounding_boxes, LassoState, Tool, ToolContext};

mod gizmos;
mod cursor_icon;

enum DragState {
    None,
    Lasso(LassoState),
    Move(elic::Vec2),
    Scale {
        pivot: elic::Vec2,
        origin: elic::Vec2,
        curr_pos: elic::Vec2
    },
    Rotate {
        pivot: elic::Vec2,
        origin: elic::Vec2,
        curr_pos: elic::Vec2
    },
    Pivot
}

pub struct SelectTool {
    select_bounding_box: Option<elic::Rect>,
    select_bounding_box_version: u64,
    select_bounding_box_transform: elic::Mat4,
    pivot: elic::Vec2,
    drag_state: DragState,
    prev_drag_mouse_pos: elic::Vec2
}

impl Default for SelectTool {
    fn default() -> Self {
        Self {
            select_bounding_box: None,
            select_bounding_box_version: 0,
            select_bounding_box_transform: elic::Mat4::IDENTITY,
            pivot: elic::Vec2::ZERO,
            drag_state: DragState::None,
            prev_drag_mouse_pos: elic::Vec2::ZERO
        }
    }
}

impl SelectTool {

    fn recalculate_bounding_box(&mut self, client: &Client, selection: &Selection) {
        let mut bounds = None;
        for stroke in selection.iter::<Stroke>() {
            let Some(stroke) = client.get(stroke) else { continue; };
            let Some(stroke_bounds) = bounding_boxes::stroke(stroke) else { continue; };
            bounds = Some(bounds.map(|bounds: elic::Rect| bounds.merge(stroke_bounds)).unwrap_or(stroke_bounds));
        }
        for fill in selection.iter::<Fill>() {
            let Some(fill) = client.get(fill) else { continue; };
            let Some(fill_bounds) = bounding_boxes::fill(fill) else { continue; };
            bounds = Some(bounds.map(|bounds: elic::Rect| bounds.merge(fill_bounds)).unwrap_or(fill_bounds));
        }
        self.select_bounding_box = bounds;
        self.select_bounding_box_version = selection.version();
        self.select_bounding_box_transform = elic::Mat4::IDENTITY;
        self.pivot = bounds.map(|bounds| bounds.center()).unwrap_or(elic::Vec2::ZERO); 
    }

    fn calc_scale_transform(pivot: elic::Vec2, origin: elic::Vec2, curr_pos: elic::Vec2, shift_down: bool, prev_transform: elic::Mat4) -> elic::Mat4 {
        let right_transformed = prev_transform.transform(elic::Vec2::X) - prev_transform.transform(elic::Vec2::ZERO);
        let angle = -right_transformed.angle_between(elic::Vec2::X);
        let unrotate = elic::Mat4::rotate(-angle);
        let rerotate = elic::Mat4::rotate( angle);

        let initial_size = unrotate.transform(origin - pivot);
        let current_size = unrotate.transform(curr_pos - pivot);
        let mut scale_factor = current_size / initial_size;
        if shift_down {
            scale_factor = elic::Vec2::splat(scale_factor.max_component());
        }
        (rerotate * elic::Mat4::scale(scale_factor) * unrotate).with_fixed_point(pivot)
    }

    fn calc_rotate_transform(pivot: elic::Vec2, origin: elic::Vec2, curr_pos: elic::Vec2, shift_down: bool) -> elic::Mat4 {
        let initial_dir = origin - pivot;
        let current_dir = curr_pos - pivot;
        let mut angle = initial_dir.angle_between(current_dir); 
        if shift_down {
            let step_size = std::f32::consts::TAU / 12.0;
            angle = step_size * (angle / step_size).round();
        }
        elic::Mat4::rotate(angle).with_fixed_point(pivot)
    }

    fn apply_transform(&mut self, client: &Client, editor: &mut EditorState, transform: elic::Mat4) {
        let mut action = Action::new(editor.action_context("Transform strokes"));
        for stroke_ptr in editor.selection.iter::<Stroke>() {
            let Some(stroke) = client.get(stroke_ptr) else { continue; };
            let new_stroke_path = stroke.stroke.0.path.map(|pt|
                malvina::StrokePoint {
                    pt: transform.transform(pt.pt),
                    pressure: pt.pressure,
                }
            );
            action.push(SetStrokeStroke {
                ptr: stroke_ptr,
                stroke_value: StrokeData(malvina::Stroke { path: new_stroke_path }),
            });
        }
        for fill_ptr in editor.selection.iter::<Fill>() {
            let Some(fill) = client.get(fill_ptr) else { continue; };
            let new_fill_paths = FillPaths(malvina::FillPaths {
                paths: fill.paths.0.paths.iter().map(|path| {
                    path.map(|pt| transform.transform(*pt))
                }).collect()
            });
            action.push(SetFillPaths {
                ptr: fill_ptr,
                paths_value: new_fill_paths,
            });
        }
        client.queue_action(action);

        self.select_bounding_box_transform = transform * self.select_bounding_box_transform;
        self.pivot = transform.transform(self.pivot);
    }

    fn curr_transform(&self, shift_down: bool, option_down: bool) -> elic::Mat4 {
        match self.drag_state {
            DragState::Move(drag) => elic::Mat4::translate(drag),
            DragState::Scale { pivot, origin, curr_pos } => {
                let scaling_pivot = if option_down {
                    self.pivot
                } else {
                    pivot
                };
                Self::calc_scale_transform(scaling_pivot, origin, curr_pos, shift_down, self.select_bounding_box_transform)
            },
            DragState::Rotate { pivot, origin, curr_pos } => {
                let rotation_pivot = if option_down {
                    pivot
                } else {
                    self.pivot
                };
                Self::calc_rotate_transform(rotation_pivot, origin, curr_pos, shift_down)
            },
            _ => elic::Mat4::IDENTITY
        }
    }

    fn bounding_box_transform(&self, shift_down: bool, option_down: bool) -> elic::Mat4 {
        self.curr_transform(shift_down, option_down) * self.select_bounding_box_transform
    }

}

keyboard_shortcut!(SelectToolShortcut, V, pierro::KeyModifiers::empty());

impl Tool for SelectTool {

    const ICON: &'static str = pierro::icons::CURSOR;

    type Shortcut = SelectToolShortcut;

    fn mouse_drag_started(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.prev_drag_mouse_pos = pos;

        let gizmos = self.calc_gizmos(
            ctx.key_modifiers.contains(pierro::KeyModifiers::SHIFT),
            ctx.key_modifiers.contains(pierro::KeyModifiers::OPTION),
            ctx.cam_zoom
        );
        if let Some(gizmos) = gizmos {
            match gizmos.get_pivot(pos, ctx.cam_zoom) {
                PotentialDragState::None => {},
                PotentialDragState::Scale(pivot) => {
                    editor.selection.keep_selection();
                    self.drag_state = DragState::Scale { pivot, origin: pos, curr_pos: pos };
                    return;
                },
                PotentialDragState::Rotate(pivot) => {
                    editor.selection.keep_selection();
                    self.drag_state = DragState::Rotate { pivot, origin: pos, curr_pos: pos };
                    return;
                },
                PotentialDragState::Pivot => {
                    self.drag_state = DragState::Pivot;
                    editor.selection.keep_selection();
                    return;
                }
            }
        }

        if let Some((x, y)) = ctx.picking_mouse_pos {
            match ctx.pick(x, y) {
                Some(obj) => {
                    if ctx.modifiable_objs.contains(&obj) {
                        if !editor.selection.is_scene_obj_selected(obj) {
                            if !editor.selection.shift_down() {
                                editor.selection.clear();
                            }
                            editor.selection.select_scene_obj(obj);
                        }
                        editor.selection.keep_selection();
                        self.drag_state = DragState::Move(elic::Vec2::ZERO);
                        return;
                    }
                },
                None => {}
            }
        }

        self.drag_state = DragState::Lasso(LassoState::from_point(pos));
    }

    fn mouse_dragged(&mut self, _editor: &mut EditorState, _ctx: &mut ToolContext, pos: malvina::Vec2) {
        match &mut self.drag_state {
            DragState::None => {},
            DragState::Lasso(lasso) => {
                lasso.add_point(pos);
            },
            DragState::Move(drag) => {
                *drag += pos - self.prev_drag_mouse_pos;
            },
            DragState::Scale { curr_pos, .. } => {
                *curr_pos = pos; 
            },
            DragState::Rotate { curr_pos, .. } => {
                *curr_pos = pos;
            },
            DragState::Pivot => {
                self.pivot = pos;
            }
        }
        self.prev_drag_mouse_pos = pos;
    }

    fn mouse_drag_stopped(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2) {
        match std::mem::replace(&mut self.drag_state, DragState::None) {
            DragState::None => {},
            DragState::Lasso(mut lasso) => {
                lasso.add_point(pos);

                for obj in lasso.find_inside(&ctx.project.client, ctx.modifiable_objs) {
                    editor.selection.select_scene_obj(obj);
                }
            },
            DragState::Move(drag) => {
                self.apply_transform(&ctx.project.client, editor, elic::Mat4::translate(drag));
            },
            DragState::Scale { pivot, origin, curr_pos } => {
                let scaling_pivot = if ctx.key_modifiers.contains(pierro::KeyModifiers::OPTION) {
                    self.pivot
                } else {
                    pivot
                };
                self.apply_transform(&ctx.project.client, editor, Self::calc_scale_transform(scaling_pivot, origin, curr_pos, ctx.key_modifiers.contains(pierro::KeyModifiers::SHIFT), self.select_bounding_box_transform));
            },
            DragState::Rotate { pivot, origin, curr_pos } => {
                let rotation_pivot = if ctx.key_modifiers.contains(pierro::KeyModifiers::OPTION) {
                    pivot
                } else {
                    self.pivot
                };
                self.apply_transform(&ctx.project.client, editor, Self::calc_rotate_transform(rotation_pivot, origin, curr_pos, ctx.key_modifiers.contains(pierro::KeyModifiers::SHIFT)));
            },
            DragState::Pivot => {}
        }
    }

    fn mouse_clicked(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, _pos: malvina::Vec2) {
        if let Some((x, y)) = ctx.picking_mouse_pos {
            match ctx.pick(x, y) {
                Some(obj) => {
                    if ctx.modifiable_objs.contains(&obj) {
                        editor.selection.extend_select_scene_obj(obj);
                    }
                }
                None => {},
            }
        }
    }

    fn tick(&mut self, editor: &mut EditorState, ctx: &mut ToolContext) {
        if editor.selection.version() != self.select_bounding_box_version {
            self.recalculate_bounding_box(&ctx.project.client, &editor.selection);
        } 

        match &self.drag_state {
            DragState::Move(_) | DragState::Scale { .. } | DragState::Rotate { .. } => {
                if editor.will_undo || editor.will_redo {
                    editor.will_undo = false;
                    self.drag_state = DragState::None;
                } else {
                    editor.preview.selection_transform = self.curr_transform(
                        ctx.key_modifiers.contains(pierro::KeyModifiers::SHIFT),
                        ctx.key_modifiers.contains(pierro::KeyModifiers::OPTION)
                    );
                    editor.preview.keep_preview = true;
                }
            },
            _ => {}
        }
    }

    fn render_overlay(&self, ctx: &mut ToolContext, rndr: &mut malvina::LayerRenderer, accent_color: elic::Color) {
        match &self.drag_state {
            DragState::Lasso(lasso) => {
                lasso.render_overlay(rndr, accent_color);
            },
            _ => {}
        }

        let gizmos = self.calc_gizmos(
            ctx.key_modifiers.contains(pierro::KeyModifiers::SHIFT),
            ctx.key_modifiers.contains(pierro::KeyModifiers::OPTION),
            ctx.cam_zoom
        );
        if let Some(gizmos) = gizmos {
            gizmos.render(rndr, accent_color);
        } 
    }

    fn cursor_icon(&self, _editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2) -> pierro::CursorIcon {
        self.cursor_icon(ctx, pos)
    }
    
}
