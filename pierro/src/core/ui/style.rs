
use std::{any::{Any, TypeId}, collections::HashMap};

pub trait Style: Any {
    type Value: Clone;

    fn default() -> Self::Value;
}

pub(crate) struct StyleStack {
    styles: HashMap<TypeId, Box<dyn Any>>,
    stack: Vec<(TypeId, Box<dyn Any>)>
}

impl StyleStack {

    pub(crate) fn new() -> Self {
        Self {
            styles: HashMap::new(),
            stack: Vec::new()
        }
    }

    pub(crate) fn get<S: Style>(&mut self) -> S::Value {
        let id = TypeId::of::<S>();
        if !self.styles.contains_key(&id) {
            self.styles.insert(id, Box::new(S::default()));
        }
        self.styles.get(&id).unwrap().downcast_ref::<S::Value>().unwrap().clone()
    }

    pub(crate) fn push<S: Style>(&mut self, style: S::Value) {
        let id = TypeId::of::<S>();
        let old_style = self.styles.insert(id, Box::new(style)).unwrap_or(Box::new(S::default()));
        self.stack.push((id, old_style));
    } 

    pub(crate) fn pop(&mut self) {
        let Some((id, style)) = self.stack.pop() else { panic!("style stack empty!"); };
        self.styles.insert(id, style); 
    }

}

#[macro_export]
macro_rules! style {
    ($name: ident, $t: ty, $default: expr) => {
        pub struct $name;

        impl ::alisa::Style for $name {
            type Value = $t;

            fn default() -> Self::Value {
                $default
            }
        }
    };
}
