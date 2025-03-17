
use crate::{Operation, OperationDyn, ProjectContext};

pub trait InvertibleOperation: Operation {

    type Inverse: InvertibleOperation<Project = Self::Project, Inverse = Self>;

    /// Get the inverse operation. 
    fn inverse(&self, context: &ProjectContext<Self::Project>) -> Option<Self::Inverse>;


}

pub trait InvertibleOperationDyn: OperationDyn {
    
    fn inverse(&self, context: &ProjectContext<Self::Project>) -> Option<Box<dyn InvertibleOperationDyn<Project = Self::Project>>>;

}

impl<O: InvertibleOperation> InvertibleOperationDyn for O {

    fn inverse(&self, context: &ProjectContext<Self::Project>) -> Option<Box<dyn InvertibleOperationDyn<Project = Self::Project>>> {
        Some(Box::new(Self::inverse(self, context)?))
    }

}
