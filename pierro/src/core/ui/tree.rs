
use std::{fmt::Debug, hash::Hash};

use crate::{hash, Axis, Color, Margin, Painter, PerAxis, Rect, Stroke, TSTransform, TextStyle, Texture, Vec2};

use super::{Id, Layout, Size};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UIRef {
    Null,
    Some(usize)
}

impl Debug for UIRef {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Some(idx) => f.debug_tuple("some").field(idx).finish(),
        }
    }

}

impl UIRef {

    pub fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }

    pub fn as_option(&self) -> Option<Self> {
        match self {
            UIRef::Null => None,
            UIRef::Some(_) => Some(*self),
        }
    } 

}

pub struct UINodeParams {
    // Layout
    pub(crate) size: PerAxis<Size>,
    pub(crate) layout: Layout,
    pub(crate) margin: Margin,
    pub(crate) interaction_margin: Margin,
    pub(crate) transform: TSTransform,

    // Styling
    pub(crate) fill: Color,
    pub(crate) rounding: f32,
    pub(crate) stroke: Stroke,
    pub(crate) clip: bool,
    pub(crate) texture: Option<Texture>,

    // Text
    pub(crate) text: Option<String>,
    pub(crate) text_style: TextStyle,

    // Id
    pub(crate) id_source: Option<u64>,

    // Input
    pub(crate) mouse: bool,
    pub(crate) scroll: bool,
    pub(crate) dnd_hover: bool,
    pub(crate) has_interaction_priority: bool,

    // Custom Behaviour 
    pub(crate) on_paint: Option<Box<dyn FnOnce(&mut Painter, Rect)>>
}

impl UINodeParams {

    pub fn new(w: Size, h: Size) -> Self {
        Self {
            size: PerAxis::new(w, h),
            layout: Layout::new(Axis::Y),
            margin: Margin::ZERO,
            interaction_margin: Margin::ZERO,
            transform: TSTransform::IDENTITY,
            fill: Color::TRANSPARENT,
            rounding: 0.0,
            stroke: Stroke::NONE,
            clip: true,
            text: None,
            text_style: TextStyle::default(),
            texture: None,
            id_source: None,
            mouse: false,
            scroll: false,
            dnd_hover: false,
            has_interaction_priority: false,
            on_paint: None
        }
    }
    
    pub fn new_per_axis(size: PerAxis<Size>) -> Self {
        Self::new(size.x, size.y)
    }

    pub fn with_size(mut self, w: Size, h: Size) -> Self {
        self.size.x = w;
        self.size.y = h;
        self
    }

    pub fn with_layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }

    pub fn with_interaction_margin(mut self, margin: Margin) -> Self {
        self.interaction_margin = margin;
        self
    }

    pub fn with_transform(mut self, transform: TSTransform) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_fill(mut self, color: Color) -> Self {
        self.fill = color;
        self
    }

    pub fn with_rounding(mut self, rounding: f32) -> Self {
        self.rounding = rounding;
        self
    }

    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = stroke;
        self
    }

    pub fn with_texture(mut self, texture: Texture) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn with_clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    pub fn no_clip(self) -> Self {
        self.with_clip(false)
    }

    pub fn with_text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_text_style(mut self, style: TextStyle) -> Self {
        self.text_style = style;
        self
    }

    pub fn with_id<H: Hash>(mut self, source: &H) -> Self {
        self.id_source = Some(hash(source));
        self
    }

    pub fn sense_mouse(mut self) -> Self {
        self.mouse = true;
        self
    }
    
    pub fn sense_scroll(mut self) -> Self {
        self.scroll = true;
        self
    }

    pub fn sense_dnd_hover(mut self) -> Self {
        self.dnd_hover = true;
        self
    }

    pub fn with_interaction_priority(mut self) -> Self {
        self.has_interaction_priority = true;
        self
    }

    pub fn on_paint<F: FnOnce(&mut Painter, Rect) + 'static>(mut self, on_paint: F) -> Self {
        self.on_paint = Some(Box::new(on_paint));
        self
    }

}

pub(crate) struct UINode {
    pub(crate) id: Id,

    // tree links
    pub(crate) next: UIRef,
    pub(crate) prev: UIRef,
    pub(crate) first_child: UIRef,
    pub(crate) last_child: UIRef,
    pub(crate) n_children: u64,
    pub(crate) parent: UIRef,

    pub(crate) params: UINodeParams,

    // layout
    pub(crate) rect: Rect,
    pub(crate) transform: TSTransform,
    pub(crate) basis_size: Vec2,
    pub(crate) frac_units: Vec2,
    
    /// The current ID seed for adding children to this node. 
    pub(crate) curr_id_seed: u64,
}

impl UINode {

    pub(crate) fn new(parent_id: Id, id_seed: u64, params: UINodeParams) -> Self {
        Self {
            id: Id(hash(&(parent_id, params.id_source.unwrap_or(id_seed)))),
            next: UIRef::Null,
            prev: UIRef::Null,
            first_child: UIRef::Null,
            last_child: UIRef::Null,
            n_children: 0,
            parent: UIRef::Null,
            params,
            rect: Rect::ZERO,
            transform: TSTransform::IDENTITY,
            basis_size: Vec2::ZERO,
            frac_units: Vec2::ONE,
            curr_id_seed: 0
        }
    }

}

/// A tree of UI nodes
pub(crate) struct UITree {
    /// All the nodes in the tree
    pub(crate) nodes: Vec<UINode>,
    /// The root node of each layer of the UI.
    /// Layers cover the entire screen and are drawn in order, allowing for popups, context menus, etc.
    /// Each layer is its own tree of nodes, with the layer node being the root.
    pub(crate) layers: Vec<UIRef>
}

impl UITree {

    pub(crate) fn new() -> Self {
        Self {
            nodes: Vec::new(),
            layers: Vec::new()
        }
    }

    pub(crate) fn get(&self, node: UIRef) -> &UINode {
        match node {
            UIRef::Null => panic!("cannot get null node ref."),
            UIRef::Some(idx) => &self.nodes[idx],
        } 
    }

    pub(crate) fn get_mut(&mut self, node: UIRef) -> &mut UINode {
        match node {
            UIRef::Null => panic!("cannot get null node ref."),
            UIRef::Some(idx) => &mut self.nodes[idx],
        } 
    }

    pub(crate) fn add_node(&mut self, node: UINode) -> UIRef {
        self.nodes.push(node); 
        UIRef::Some(self.nodes.len() - 1)
    }
    
    pub(crate) fn add_layer(&mut self, size: Vec2) -> UIRef {
        let layer = self.add_node(UINode::new(
            Id(0),
            self.layers.len() as u64,
            UINodeParams::new(Size::px(size.x), Size::px(size.y)) 
        ));
        self.layers.push(layer);
        layer
    }

}
