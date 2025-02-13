
use crate::{PaintRect, PaintText, Painter};

use super::{UIRef, UITree};

impl UITree {

    fn paint_node(&mut self, painter: &mut Painter, node_ref: UIRef) {

        let node = self.get_mut(node_ref); 

        painter.push_transform(node.transform);

        if node.params.fill.a > 0.0 || (node.params.stroke.color.a > 0.0 && node.params.stroke.width > 0.0) {
            painter.rect(PaintRect::new(node.rect, node.params.fill).with_rounding(node.params.rounding).with_stroke(node.params.stroke));
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
