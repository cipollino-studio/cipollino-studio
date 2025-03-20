
use std::fmt::Debug;

use crate::{hash, Rect, TSTransform, Vec2};

use super::{Id, Size, UINodeParams};

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
            UINodeParams::new(Size::px(size.x), Size::px(size.y)).reject_focus() 
        ));
        self.layers.push(layer);
        layer
    }

}
