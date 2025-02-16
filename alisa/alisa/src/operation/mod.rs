
use std::any::{type_name, Any, TypeId};

use crate::{DeserializationContext, Project, ProjectContext, Serializable, SerializationContext};

mod common;

mod delta;
pub use delta::*;

/// An operation performed on the project. 
/// Operations can be inverted for undo/redo. 
/// Note that when collaborating, undoing an operation and redoing might not return to the original state of the project. 
pub trait Operation: Sized + Any + Serializable<Self::Project> + Send + Sync {

    type Project: Project;
    type Inverse: Operation<Project = Self::Project, Inverse = Self>;

    /// The name of the operation, used for collab messages. MAKE SURE THIS IS UNIQUE FOR ALL OPERATIONS!
    const NAME: &'static str;

    /// Perform the operation.
    fn perform(&self, recorder: &mut Recorder<'_, Self::Project>); 
    /// Get the inverse operation. 
    fn inverse(&self, context: &ProjectContext<Self::Project>) -> Option<Self::Inverse>;

}

/// Shim trait for turning an operation into a dyn object
pub(crate) trait OperationDyn: Send + Sync {
    type Project: Project;

    fn perform(&self, recorder: &mut Recorder<'_, Self::Project>);
    fn inverse(&self, context: &ProjectContext<Self::Project>) -> Option<Box<dyn OperationDyn<Project = Self::Project>>>;
    fn name(&self) -> &'static str;
    fn serialize(&self) -> rmpv::Value;
}

impl<O: Operation + Serializable<O::Project>> OperationDyn for O {
    type Project = O::Project;

    fn perform(&self, recorder: &mut Recorder<'_, Self::Project>) {
        self.perform(recorder);
    }

    fn inverse(&self, context: &ProjectContext<Self::Project>) -> Option<Box<dyn OperationDyn<Project = Self::Project>>> {
        if let Some(inverse) = <Self as Operation>::inverse(self, context) {
            return Some(Box::new(inverse)); 
        }
        None
    }

    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn serialize(&self) -> rmpv::Value {
        self.serialize(&SerializationContext::shallow())
    }

}

/// A kind of operation, stored as a struct in `Project::OPERATIONS`.
pub struct OperationKind<P: Project> {
    pub(crate) name: &'static str,
    pub(crate) deserialize: fn(&rmpv::Value) -> Option<Box<dyn Any>>,
    pub(crate) perform: fn(Box<dyn Any>, &mut Recorder<'_, P>),

    #[cfg(debug_assertions)]
    pub(crate) type_id: fn() -> TypeId,
    #[cfg(debug_assertions)]
    pub(crate) type_name: fn() -> &'static str
}

impl<P: Project> OperationKind<P> {

    pub const fn from<O: Operation<Project = P>>() -> Self {
        Self {
            name: O::NAME,
            deserialize: |data| {
                Some(Box::new(O::deserialize(data, &mut DeserializationContext::data())?))
            },
            perform: |operation, recorder| {
                let Ok(operation) = operation.downcast::<O>() else { return; };
                operation.perform(recorder);
            },
            #[cfg(debug_assertions)]
            type_id: || TypeId::of::<O>(),
            #[cfg(debug_assertions)]
            type_name: || type_name::<O>()
        }
    }

}

/// An operation that was not yet confirmed by the server. Used for moving backwards/forwards in time for conflict resolution.  
pub(crate) struct UnconfirmedOperation<P: Project> {
    pub(crate) operation: Box<dyn OperationDyn<Project = P>>,
    pub(crate) deltas: Vec<Box<dyn Delta<Project = P>>> 
}
