
use crate::Axis;
use super::{DockingNodeId, DockingState, DockingTab, DockingTree};

pub enum DockingLayout<Tab: DockingTab> {
    Tabs(Vec<Tab>),
    Split(Axis, Vec<(f32, DockingLayout<Tab>)>)
}

impl<Tab: DockingTab> DockingLayout<Tab> {

    fn into_tree(self, tree: &mut DockingTree<Tab>, parent: DockingNodeId) -> DockingNodeId {
        match self {
            DockingLayout::Tabs(tabs) => {
                tree.add_tabs(parent, tabs)
            },
            DockingLayout::Split(axis, children) => {
                let split = tree.add_split(parent, axis);
                for (size, child) in children {
                    let child = child.into_tree(tree, split);
                    tree.get_split_mut(split).unwrap().nodes.push((size, child));
                }
                split
            }
        }
    }

    pub fn into_state(self) -> DockingState<Tab> {
        let mut tree = DockingTree::empty();
        let root = self.into_tree(&mut tree, DockingNodeId::NULL);
        tree.root = root;
        DockingState { tree }
    }

}
