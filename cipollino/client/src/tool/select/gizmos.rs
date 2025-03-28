
use super::SelectTool;

pub(super) struct FreeTransformGizmos {
    pub tl: elic::Vec2,
    pub tr: elic::Vec2,
    pub bl: elic::Vec2,
    pub br: elic::Vec2,
}

impl SelectTool {

    pub(super) fn calc_gizmos(&self) -> Option<FreeTransformGizmos> {
        let bounding_box = self.select_bounding_box?;
        let transform = self.bounding_box_transform();
        let tl = transform.transform(bounding_box.tl());
        let tr = transform.transform(bounding_box.tr());
        let bl = transform.transform(bounding_box.bl());
        let br = transform.transform(bounding_box.br());
        Some(FreeTransformGizmos {
            tl,
            tr,
            bl,
            br,
        })
    }

}

impl FreeTransformGizmos {

    pub const RADIUS: f32 = 3.5;
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
    }

    pub fn get_resizing_pivot(&self, mouse_pos: elic::Vec2, zoom: f32) -> Option<elic::Vec2> {
        let potential_pivots = [
            (self.br, mouse_pos.distance(self.tl)),
            (self.bl, mouse_pos.distance(self.tr)),
            (self.tr, mouse_pos.distance(self.bl)),
            (self.tl, mouse_pos.distance(self.br)),
        ];
        let (pivot, dist) = potential_pivots.iter().min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))?;
        if *dist > Self::INTERACTION_RADIUS / zoom {
            return None;
        }
        Some(*pivot)
    }

}
