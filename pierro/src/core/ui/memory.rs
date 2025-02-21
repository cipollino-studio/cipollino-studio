
use std::{any::{Any, TypeId}, collections::HashMap, u64};

use super::UITree;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Id(pub(crate) u64);

pub struct Memory {
    pub(crate) layer_ids: Vec<Id>,
    focused: Option<Id>,
    dnd_payload: Option<Box<dyn Any>>,
    memory: HashMap<(Id, TypeId), Box<dyn Any>>
}

impl Memory {

    pub(crate) fn new() -> Self {
        Self {
            layer_ids: Vec::new(),
            focused: None,
            dnd_payload: None,
            memory: HashMap::new(),
        }
    }

    /// Focus on a node
    pub fn request_focus(&mut self, node: Id) {
        self.focused = Some(node);
    }

    /// Release focus
    pub fn release_focus(&mut self) {
        self.focused = None;
    }

    /// Check if a node focused 
    pub fn is_focused(&self, node: Id) -> bool {
        self.focused == Some(node)
    }

    /// Get the focused node
    /// Returns `None` if no node is focused
    pub fn get_focus(&self) -> Option<Id> {
        self.focused
    }

    pub fn set_dnd_payload<T: Any>(&mut self, payload: T) {
        self.dnd_payload = Some(Box::new(payload)); 
    }

    pub fn has_dnd_payload(&self) -> bool {
        self.dnd_payload.is_some()
    }

    pub fn has_dnd_payload_of_type<T: Any>(&self) -> bool {
        match &self.dnd_payload {
            Some(payload) => {
                (&**payload).type_id() == TypeId::of::<T>()
            },
            None => false,
        } 
    }

    pub fn get_dnd_payload<T: Any>(&mut self) -> Option<&T> {
        match &self.dnd_payload {
            Some(payload) => payload.downcast_ref(),
            None => None,
        }
    }

    pub fn take_dnd_payload<T: Any>(&mut self) -> Option<T> {
        if !self.has_dnd_payload_of_type::<T>() {
            return None;
        }
        Some(*self.dnd_payload.take().unwrap().downcast().unwrap())
    }

    pub fn clear_dnd_payload(&mut self) {
        self.dnd_payload = None;
    }

    /// Get a reference to the data of type `T` for a certain node.
    /// Sets node's data to `T::default()` if it didn't previously exist.
    pub fn get<T: Default + Any>(&mut self, id: Id) -> &mut T {
        let key = (id, TypeId::of::<T>());
        if !self.memory.contains_key(&key) {
            self.memory.insert(key, Box::new(T::default()));
        }
        self.memory.get_mut(&key).unwrap().downcast_mut().unwrap()
    }

    /// Get a reference to the data of type `T` for a certain node if it exists.
    pub fn get_opt<T: Any>(&mut self, id: Id) -> Option<&mut T> {
        let key = (id, TypeId::of::<T>());
        self.memory.get_mut(&key).map(|data| data.downcast_mut().unwrap())
    }

    /// Insert the data for a certain node.
    pub fn insert<T: Any>(&mut self, id: Id, data: T) {
        let key = (id, TypeId::of::<T>());
        self.memory.insert(key, Box::new(data));
    }

    /// Remove the data of type `T` for a certain node. Returns the data.
    pub fn remove<T: Any>(&mut self, id: Id) -> Option<T> {
        let key = (id, TypeId::of::<T>());
        self.memory.remove(&key).map(|data| *data.downcast().unwrap())
    }

    /// Does a node have data of a certain type in memory?
    pub fn has<T: Any>(&self, id: Id) -> bool {
        let key = (id, TypeId::of::<T>());
        self.memory.contains_key(&key)
    }

    /// Iterate through all data in memory of type `T`
    pub fn iter_mut<T: Default + Any>(&mut self) -> impl Iterator<Item = (Id, &mut T)> {
        let typ_id = TypeId::of::<T>();
        self.memory.iter_mut().filter_map(move |((id, typ), data)| if *typ == typ_id {
            Some((*id, data.downcast_mut().unwrap()))
        } else {
            None
        })
    }

    /// Remove memory for all ui nodes that are not in the ui tree.
    pub(crate) fn garbage_collect(&mut self, tree: &UITree) {
        let mut live_nodes = ahash::AHashSet::new();
        for node in &tree.nodes {
            live_nodes.insert(node.id);
        }

        self.memory.retain(|(id, _), _| live_nodes.contains(id));

        if let Some(focused) = self.focused {
            if !live_nodes.contains(&focused) {
                self.focused = None;
            }
        }
    }

}
