
use super::SelectTool;

pub(super) struct FreeTransformGizmos {
    pub tl: elic::Vec2,
    pub tr: elic::Vec2,
    pub bl: elic::Vec2,
    pub br: elic::Vec2,

    pub rotate_tl: elic::Vec2,
    pub rotate_tr: elic::Vec2,
    pub rotate_bl: elic::Vec2,
    pub rotate_br: elic::Vec2,

    pub pivot: elic::Vec2,
}

impl SelectTool {

    pub(super) fn calc_gizmos(&self, shift_down: bool, option_down: bool, zoom: f32) -> Option<FreeTransformGizmos> {
        let bounding_box = self.select_bounding_box?;
        let transform = self.bounding_box_transform(shift_down, option_down);
        let tl = transform.transform(bounding_box.tl());
        let tr = transform.transform(bounding_box.tr());
        let bl = transform.transform(bounding_box.bl());
        let br = transform.transform(bounding_box.br());

        let r = (tr - tl).normalize() * FreeTransformGizmos::ROTATION_HANDLE_DISTANCE / zoom;
        let l = -r;
        let u = (tl - bl).normalize() * FreeTransformGizmos::ROTATION_HANDLE_DISTANCE / zoom;
        let d = -u;

        let rotate_tl = tl + l + u; 
        let rotate_tr = tr + r + u; 
        let rotate_bl = bl + l + d; 
        let rotate_br = br + r + d; 

        Some(FreeTransformGizmos {
            tl,
            tr,
            bl,
            br,
            rotate_tl,
            rotate_tr,
            rotate_bl,
            rotate_br,
            pivot: self.curr_transform(shift_down, option_down).transform(self.pivot)
        })
    }

}

#[derive(Clone, Copy)]
pub(super) enum PotentialDragState {
    None,
    Scale(elic::Vec2),
    Rotate(elic::Vec2),
    Pivot
}

impl FreeTransformGizmos {

    pub const RADIUS: f32 = 3.5;
    pub const ROTATION_HANDLE_DISTANCE: f32 = Self::RADIUS * 4.0;
    pub const INTERACTION_RADIUS: f32 = 2.0 * Self::RADIUS;

    fn render_circle(rndr: &mut malvina::LayerRenderer, accent_color: elic::Color, pos: elic::Vec2) {
        rndr.overlay_circle(pos, Self::RADIUS, accent_color);
        rndr.overlay_circle(pos, Self::RADIUS - 1.0, elic::Color::WHITE);
    }

    pub fn render(&self, rndr: &mut malvina::LayerRenderer, accent_color: elic::Color) {
        rndr.overlay_line(self.tl, self.tr, accent_color);
        rndr.overlay_line(self.tr, self.br, accent_color);
        rndr.overlay_line(self.br, self.bl, accent_color);
        rndr.overlay_line(self.bl, self.tl, accent_color);

        Self::render_circle(rndr, accent_color, self.tl);
        Self::render_circle(rndr, accent_color, self.tr);
        Self::render_circle(rndr, accent_color, self.bl);
        Self::render_circle(rndr, accent_color, self.br);

        Self::render_circle(rndr, accent_color, self.rotate_tl);
        Self::render_circle(rndr, accent_color, self.rotate_tr);
        Self::render_circle(rndr, accent_color, self.rotate_bl);
        Self::render_circle(rndr, accent_color, self.rotate_br);

        Self::render_circle(rndr, accent_color, self.pivot);
    }

    pub fn get_pivot(&self, mouse_pos: elic::Vec2, zoom: f32) -> PotentialDragState {
        let potential_pivots = [
            (PotentialDragState::Scale(self.br), mouse_pos.distance(self.tl)),
            (PotentialDragState::Scale(self.bl), mouse_pos.distance(self.tr)),
            (PotentialDragState::Scale(self.tr), mouse_pos.distance(self.bl)),
            (PotentialDragState::Scale(self.tl), mouse_pos.distance(self.br)),

            (PotentialDragState::Rotate(self.br), mouse_pos.distance(self.rotate_tl)),
            (PotentialDragState::Rotate(self.bl), mouse_pos.distance(self.rotate_tr)),
            (PotentialDragState::Rotate(self.tr), mouse_pos.distance(self.rotate_bl)),
            (PotentialDragState::Rotate(self.tl), mouse_pos.distance(self.rotate_br)),

            (PotentialDragState::Pivot, mouse_pos.distance(self.pivot))
        ];
        let Some((pivot, dist)) = potential_pivots.iter().min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)) else {
            return PotentialDragState::None;
        };
        if *dist > Self::INTERACTION_RADIUS / zoom {
            return PotentialDragState::None;
        }
        *pivot
    }

}
