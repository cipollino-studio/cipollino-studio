
use crate::{vec2, PaintRect, PaintText, Painter, Vec2};

use super::{TextureMapMode, UIRef, UITree};

impl UITree {

    fn calc_uvs(texture_map: TextureMapMode, texture_size: Vec2, target_size: Vec2) -> (Vec2, Vec2) {
        match texture_map {
            TextureMapMode::Scale => (Vec2::ZERO, Vec2::ONE),
            TextureMapMode::Fit => {
                let scale = target_size / texture_size;
                let scale = scale / scale.max_component();
                let uv_min = Vec2::splat(0.5) - scale * 0.5; 
                let uv_max = Vec2::splat(0.5) + scale * 0.5;
                (uv_min, uv_max)
            },
            TextureMapMode::Cover => {
                let mut scale = target_size / texture_size;
                if scale.max_component() > 1.0 {
                    scale /= scale.max_component();
                }
                let uv_min = Vec2::splat(0.5) - scale * 0.5; 
                let uv_max = Vec2::splat(0.5) + scale * 0.5;
                (uv_min, uv_max)
            },
        }
    }

    fn paint_node(&mut self, painter: &mut Painter, node_ref: UIRef) {

        let node = self.get_mut(node_ref); 

        painter.push_transform(node.transform);

        if node.params.fill.a > 0.0 || (node.params.stroke.color.a > 0.0 && node.params.stroke.width > 0.0) {
            let mut rect = PaintRect::new(node.rect, node.params.fill).with_rounding(node.params.rounding).with_stroke(node.params.stroke); 
            if let Some(texture) = node.params.texture.take() {
                let (uv_min, uv_max) = Self::calc_uvs(
                    node.params.texture_map,
                    vec2(texture.width() as f32, texture.height() as f32),
                    node.rect.size() * painter.scale_factor()
                );
                rect = rect.with_texture(texture).with_uv(uv_min, uv_max);
            }
            painter.rect(rect);
        }

        if let Some(text) = node.params.text.take() {
            let text_rect = node.params.margin.apply(node.rect);
            painter.text(PaintText::new(text, node.params.text_style, text_rect));
        }

        if node.params.clip {
            painter.push_clip_rect(node.rect);
        }

        let mut child = node.first_child;
        while child.is_some() {
            self.paint_node(painter, child);
            child = self.get(child).next;
        }

        let node = self.get_mut(node_ref); 
        if let Some(on_paint) = node.params.on_paint.take() {
            on_paint(painter, node.rect);
        } 

        if node.params.clip {
            painter.pop_clip_rect();
        }
        painter.pop_transform();
    }

    pub(crate) fn paint(&mut self, painter: &mut Painter) {
        for layer in self.layers.clone() {
            self.paint_node(painter, layer);
        }
    }

}
