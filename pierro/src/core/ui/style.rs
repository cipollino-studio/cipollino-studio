
use std::{any::{Any, TypeId}, collections::HashMap};

pub(crate) struct Style {
    styles: HashMap<TypeId, Box<dyn Any>>,
    stack: Vec<(TypeId, Box<dyn Any>)>
}

impl Style {

    pub(crate) fn new() -> Self {
        Self {
            styles: HashMap::new(),
            stack: Vec::new()
        }
    }

    pub(crate) fn get<T: Default + Any>(&mut self) -> &T {
        let id = TypeId::of::<T>();
        if !self.styles.contains_key(&id) {
            self.styles.insert(id,Box::new(T::default()));
        }
        self.styles.get(&id).unwrap().downcast_ref().unwrap()
    }

    pub(crate) fn push<T: Default + Any>(&mut self, style: T) {
        let id = TypeId::of::<T>();
        let old_style = self.styles.insert(id, Box::new(style)).unwrap_or(Box::new(T::default()));
        self.stack.push((id, old_style));
    } 

    pub(crate) fn pop(&mut self) {
        let Some((id, style)) = self.stack.pop() else { panic!("style stack empty!"); };
        self.styles.insert(id, style); 
    }

}
