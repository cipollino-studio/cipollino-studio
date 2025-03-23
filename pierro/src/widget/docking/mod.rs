
mod render;
mod command;
mod layout;
pub use layout::*;

use std::{collections::HashMap, usize};

use crate::{Axis, UI};

pub trait DockingTab: Sized {

    type Context;

    fn title(&self) -> String;
    fn render(&mut self, ui: &mut UI, context: &mut Self::Context);

    fn add_tab_dropdown<F: FnMut(Self)>(ui: &mut UI, add_tab: F, context: &mut Self::Context);
    
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct Tabs<Tab: DockingTab> {
    tabs: Vec<Tab>,
    active_tab: usize
}

impl<Tab:DockingTab> Tabs<Tab> {

    pub fn new(tabs: Vec<Tab>) -> Self {
        Self {
            tabs: tabs,
            active_tab: 0,
        }
    }

}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct DockingNodeId(usize);

impl DockingNodeId {

    const NULL: Self = Self(usize::MAX);

    fn is_null(&self) -> bool {
        *self == Self::NULL
    }

}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct Split {
    nodes: Vec<(f32, DockingNodeId)>,
    direction: Axis
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum DockingNodeKind<Tab: DockingTab> {
    Tabs(Tabs<Tab>),
    Split(Split) 
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct DockingNode<Tab: DockingTab> {
    parent: DockingNodeId,
    kind: DockingNodeKind<Tab>
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct DockingTree<Tab: DockingTab> {
    nodes: HashMap<DockingNodeId, DockingNode<Tab>>,
    curr_id: usize,
    root: DockingNodeId 
}

impl<Tab: DockingTab> DockingTree<Tab> {

    fn empty() -> Self {
        Self {
            nodes: HashMap::new(),
            curr_id: 1,
            root: DockingNodeId::NULL,
        }
    }

    fn new(tabs: Vec<Tab>) -> Self {
        let mut nodes = HashMap::new();
        let root = DockingNodeId(0);
        nodes.insert(root, DockingNode {
            parent: DockingNodeId::NULL,
            kind: DockingNodeKind::Tabs(Tabs::new(tabs)),
        });
        Self {
            nodes,
            curr_id: 1,
            root
        }
    }

    fn get(&self, id: DockingNodeId) -> Option<&DockingNode<Tab>> {
        self.nodes.get(&id)
    }

    fn get_mut(&mut self, id: DockingNodeId) -> Option<&mut DockingNode<Tab>> {
        self.nodes.get_mut(&id)
    }

    fn add_node(&mut self, node: DockingNode<Tab>) -> DockingNodeId {
        let id = DockingNodeId(self.curr_id);
        self.curr_id += 1;
        self.nodes.insert(id, node);
        id
    }

    fn delete_node(&mut self, node: DockingNodeId) {
        self.nodes.remove(&node);
    }

    fn get_tabs(&self, id: DockingNodeId) -> Option<&Tabs<Tab>> { 
        let node = self.get(id)?;
        match &node.kind {
            DockingNodeKind::Tabs(tabs) => Some(tabs),
            DockingNodeKind::Split { .. } => None,
        }
    }

    fn get_tabs_mut(&mut self, id: DockingNodeId) -> Option<&mut Tabs<Tab>> { 
        let node = self.get_mut(id)?;
        match &mut node.kind {
            DockingNodeKind::Tabs(tabs) => Some(tabs),
            DockingNodeKind::Split { .. } => None,
        }
    }

    fn add_tabs(&mut self, parent: DockingNodeId, tabs: Vec<Tab>) -> DockingNodeId {
        self.add_node(DockingNode {
            parent,
            kind: DockingNodeKind::Tabs(Tabs::new(tabs)),
        })
    }

    fn get_split(&self, id: DockingNodeId) -> Option<&Split> {
        let node = self.get(id)?;
        match &node.kind {
            DockingNodeKind::Tabs { .. } => None,
            DockingNodeKind::Split(split) => Some(split),
        }
    }

    fn get_split_mut(&mut self, id: DockingNodeId) -> Option<&mut Split> {
        let node = self.get_mut(id)?;
        match &mut node.kind {
            DockingNodeKind::Tabs { .. } => None,
            DockingNodeKind::Split(split) => Some(split),
        }
    }

    fn add_split(&mut self, parent: DockingNodeId, direction: Axis) -> DockingNodeId {
        self.add_node(DockingNode {
            parent,
            kind: DockingNodeKind::Split(Split {
                nodes: Vec::new(),
                direction
            }),
        })
    }

    fn get_parent(&self, id: DockingNodeId) -> Option<DockingNodeId> {
        Some(self.get(id)?.parent)
    }

    fn set_parent(&mut self, id: DockingNodeId, parent: DockingNodeId) -> Option<()> {
        self.get_mut(id)?.parent = parent;
        Some(())
    } 

}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct DockingState<Tab: DockingTab> {
    tree: DockingTree<Tab>
}

impl<Tab: DockingTab> DockingState<Tab> {

    pub fn new(tabs: Vec<Tab>) -> Self {
        Self {
            tree: DockingTree::new(tabs)
        }
    }

}
