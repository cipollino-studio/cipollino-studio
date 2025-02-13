
use crate::Axis;
use super::{DockingNodeId, DockingNodeKind, DockingTab, DockingTree};

pub(super) struct TabDragSource {
    pub(super) node_id: DockingNodeId,
    pub(super) tab_idx: usize
}

pub(super) enum DockingCommand<Tab: DockingTab> {
    MoveTab {
        from: TabDragSource,
        to: DockingNodeId
    },
    AddTab {
        tab: Tab,
        to: DockingNodeId 
    },
    CloseTab {
        tab: TabDragSource
    },
    Split {
        tab: TabDragSource,
        to: DockingNodeId,
        direction: Axis,
        max: bool
    },
    MoveSplit {
        node_id: DockingNodeId,
        child_idx: usize,
        amount: f32,
        min_size: f32
    }
}

impl<Tab: DockingTab> DockingTree<Tab> {

    fn delete_tabs(&mut self, id: DockingNodeId) -> Option<()> {
        let parent = self.get_parent(id)?;
        if parent.is_null() {
            return None;
        }
        self.delete_node(id);
        let parent_split = self.get_split_mut(parent)?;
        parent_split.nodes.retain(|(_, other_id)| id != *other_id);

        // If the parent split now only has one node, get rid of it
        if parent_split.nodes.len() == 1 {
            let child = parent_split.nodes[0].1;

            let parent_parent = self.get_parent(parent)?;
            self.delete_node(parent);
            if parent_parent.is_null() {
                self.root = child;
            } else {
                let parent_parent_split = self.get_split_mut(parent_parent)?;
                for (_, other) in &mut parent_parent_split.nodes {
                    if *other == parent {
                        *other = child;
                    }
                }
            }
            self.set_parent(child, parent_parent);
        }

        Some(())
    }

    fn take_tab(&mut self, tab_source: TabDragSource) -> Option<Tab> {
        let parent = self.get_parent(tab_source.node_id)?;
        let from_node = self.nodes.get_mut(&tab_source.node_id)?;
        let DockingNodeKind::Tabs(tabs) = &mut from_node.kind else { return None; }; 
        if tab_source.tab_idx >= tabs.tabs.len() {
            return None;
        }

        // Don't let us take the last tab from the root node 
        if parent.is_null() && tabs.tabs.len() == 1 {
            return None;
        }

        let tab = tabs.tabs.remove(tab_source.tab_idx);

        if tabs.tabs.is_empty() {
            self.delete_tabs(tab_source.node_id);
        }

        Some(tab)
    }

    fn move_tab(&mut self, from: TabDragSource, to: DockingNodeId) -> Option<()> {
        if from.node_id == to && self.get_tabs(to)?.tabs.len() == 1 {
            return Some(());
        }
        let tab = self.take_tab(from)?;
        let to_tabs = self.get_tabs_mut(to)?;
        to_tabs.tabs.push(tab);
        Some(())
    }

    fn add_tab(&mut self, tab: Tab, to: DockingNodeId) -> Option<()> {
        let to_tabs = self.get_tabs_mut(to)?;
        to_tabs.tabs.push(tab);
        Some(())
    }

    fn split(&mut self, tab: TabDragSource, to: DockingNodeId, direction: Axis, max: bool) -> Option<()> {
        if tab.node_id == to && self.get_tabs(to)?.tabs.len() == 1 {
            return Some(());
        }

        let tab = self.take_tab(tab)?;
        let tabs = self.add_tabs(DockingNodeId::NULL, vec![tab]);

        let to_parent = self.get_parent(to)?;

        if to_parent.is_null() { // Case 1: the tabs node we're splitting is the root of the docking tree
            let new_root = self.add_split(DockingNodeId::NULL, direction);
            self.root = new_root;
            self.set_parent(to, new_root);
            self.set_parent(tabs, new_root);
            let split = self.get_split_mut(new_root)?;
            if max {
                split.nodes.push((1.0, to));
                split.nodes.push((1.0, tabs));
            } else {
                split.nodes.push((1.0, tabs));
                split.nodes.push((1.0, to));
            }
        } else if self.get_split(to_parent)?.direction != direction { // Case 2: We're splitting in a perpendicular direction, so we need a new split node
            let new_split = self.add_split(to_parent, direction);
            for (_, other) in &mut self.get_split_mut(to_parent)?.nodes {
                if *other == to {
                    *other = new_split;
                }
            }

            self.set_parent(to, new_split);
            self.set_parent(tabs, new_split);
            let split = self.get_split_mut(new_split)?;
            if max {
                split.nodes.push((1.0, to));
                split.nodes.push((1.0, tabs));
            } else {
                split.nodes.push((1.0, tabs));
                split.nodes.push((1.0, to));
            }
        } else { // Case 3: We're splitting in the same direction, so no new split is neede
            let parent_split = self.get_split_mut(to_parent)?;
            let to_idx = parent_split.nodes.iter().position(|(_, id)| *id == to)?;

            if max {
                parent_split.nodes.insert(to_idx + 1, (parent_split.nodes[to_idx].0 / 2.0, tabs));
                parent_split.nodes[to_idx].0 /= 2.0;
            } else {
                parent_split.nodes.insert(to_idx, (parent_split.nodes[to_idx].0 / 2.0, tabs));
                parent_split.nodes[to_idx + 1].0 /= 2.0;
            }
            
            self.set_parent(tabs, to_parent);
        }
        
        Some(())
    }

    fn move_split(&mut self, node_id: DockingNodeId, child_idx: usize, amount: f32, min_size: f32) -> Option<()> {
        let split = self.get_split_mut(node_id)?;

        let total_size = split.nodes[child_idx].0 + split.nodes[child_idx + 1].0;
        let min_size = min_size.min(total_size / 2.0);

        let desired_size_0 = (split.nodes[child_idx].0 + amount).max(min_size);
        let desired_size_1 = (split.nodes[child_idx + 1].0 - amount).max(min_size);

        if desired_size_0 == min_size {
            split.nodes[child_idx].0 = min_size;
            split.nodes[child_idx + 1].0 = total_size - min_size;
        } else if desired_size_1 == min_size {
            split.nodes[child_idx].0 = total_size - min_size;
            split.nodes[child_idx + 1].0 = min_size;
        } else {
            split.nodes[child_idx].0 = desired_size_0;
            split.nodes[child_idx + 1].0 = desired_size_1;
        }

        Some(())
    }

    pub(super) fn execute_command(&mut self, command: DockingCommand<Tab>) {
        match command {
            DockingCommand::MoveTab { from, to } => {
                self.move_tab(from, to);
            },
            DockingCommand::AddTab { tab, to } => {
                self.add_tab(tab, to);
            },
            DockingCommand::CloseTab { tab } => {
                self.take_tab(tab);
            },
            DockingCommand::Split { tab, to, direction, max } => {
                self.split(tab, to, direction, max);
            },
            DockingCommand::MoveSplit { node_id, child_idx, amount, min_size } => {
                self.move_split(node_id, child_idx, amount, min_size);
            }
        }
    }

}
