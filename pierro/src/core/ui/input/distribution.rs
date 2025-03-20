
use crate::{Id, LayoutMemory, Memory, Sense, Vec2};

use super::{Input, Interaction, MouseButton};

fn find_interacted_node(memory: &mut Memory, node: Id, pos: Vec2, criteria: Sense) -> Option<Id> {
    let layout_mem = memory.get::<LayoutMemory>(node);
    if !layout_mem.interaction_rect.contains(pos) && layout_mem.clip {
        return None;
    }

    // Check priority nodes first
    let mut child = memory.get::<LayoutMemory>(node).first_child;
    while let Some(child_id) = child {
        if memory.get::<LayoutMemory>(child_id).sense.contains(Sense::INTERACTION_PRIORITY) {
            if let Some(node) = find_interacted_node(memory, child_id, pos, criteria) {
                return Some(node);
            }
        }
        child = memory.get::<LayoutMemory>(child_id).next;
    } 

    // Then check non-priority nodes
    let mut child = memory.get::<LayoutMemory>(node).first_child;
    while let Some(child_id) = child {
        if !memory.get::<LayoutMemory>(child_id).sense.contains(Sense::INTERACTION_PRIORITY) {
            if let Some(node) = find_interacted_node(memory, child_id, pos, criteria) {
                return Some(node);
            }
        }
        child = memory.get::<LayoutMemory>(child_id).next;
    }

    let layout_mem = memory.get::<LayoutMemory>(node);
    if layout_mem.interaction_rect.contains(pos) && layout_mem.sense.contains(criteria) {
        return Some(node);
    }

    None

}

/// Find the node hovered at a given position in screen space
fn find_hover_node(memory: &mut Memory, node: Id, pos: Vec2) -> Option<Id> {
    find_interacted_node(memory, node, pos, Sense::MOUSE)
}

/// Find the scrollable at a given position in screen space
fn find_scrollable_node(memory: &mut Memory, node: Id, pos: Vec2) -> Option<Id> {
    find_interacted_node(memory, node, pos, Sense::SCROLL)
}

/// Find the dnd hovered node at a given position in screen space
fn find_dnd_hovered_node(memory: &mut Memory, node: Id, pos: Vec2) -> Option<Id> {
    find_interacted_node(memory, node, pos, Sense::DND_HOVER)
}

impl Input {

    /// Distribute the input to nodes, taking foucs into account.
    pub(crate) fn distribute(&mut self, memory: &mut Memory) {

        if let Some(focused_node) = memory.get_focus() {

            // Take away focus if we clicked outside the focused node
            let focused_node_memory = memory.get::<LayoutMemory>(focused_node);
            if let Some(mouse_pos) = self.mouse_pos {
                if (self.l_mouse.pressed() || self.r_mouse.pressed()) && !focused_node_memory.interaction_rect.contains(mouse_pos) {
                    memory.release_focus();
                }
            }

            // Take away focus if the focused node rejects focus 
            let focused_node_memory = memory.get::<LayoutMemory>(focused_node);
            if focused_node_memory.sense.contains(Sense::REJECT_FOCUS) {
                memory.release_focus();
            }
        }

        let layer_ids = memory.layer_ids.clone();
        let hovered_node = memory.get_focus().or_else(|| {
            let mouse_pos = self.mouse_pos?;
            for layer in layer_ids.iter().rev() { 
                if let Some(hovered_node) = find_hover_node(memory, *layer, mouse_pos) {
                    return Some(hovered_node)
                }
            }
            None
        });
        let scrollable_node = (|| {
            let mouse_pos = self.mouse_pos?;
            for layer in layer_ids.iter().rev() { 
                if let Some(scrollable_node) = find_scrollable_node(memory, *layer, mouse_pos) {
                    return Some(scrollable_node)
                }
            }
            None
        })();
        let dnd_hovered_node = (|| {
            let mouse_pos = self.mouse_pos?;
            for layer in layer_ids.iter().rev() { 
                if let Some(scrollable_node) = find_dnd_hovered_node(memory, *layer, mouse_pos) {
                    return Some(scrollable_node)
                }
            }
            None
        })();

        let keyboard_captured_node = memory.get_focus().and_then(|focused| {
            if memory.get::<LayoutMemory>(focused).sense.contains(Sense::KEYBOARD) {
                Some(focused)
            } else {
                None
            }
        });

        for (id, interaction) in memory.iter_mut::<Interaction>() {
            let hovered = Some(id) == hovered_node;
            let scrollable = Some(id) == scrollable_node;
            interaction.hovered = hovered;
            interaction.l_mouse = if hovered { self.l_mouse } else { MouseButton::new() };
            interaction.r_mouse = if hovered { self.r_mouse } else { MouseButton::new() };
            interaction.scroll = if scrollable { self.scroll } else { Vec2::ZERO };
            interaction.dnd_hovered = Some(id) == dnd_hovered_node;
            interaction.keyboard_captured = Some(id) == keyboard_captured_node;
        }

        // If we're not holding the mouse down, we can't be drag and dropping anything
        if !self.l_mouse.down() && !self.l_mouse.released() {
            memory.clear_dnd_payload();
        }

        self.keyboard_captured = keyboard_captured_node.is_some();
    }

}
