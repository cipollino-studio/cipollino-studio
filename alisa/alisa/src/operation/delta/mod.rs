
mod common;
pub use common::*;

use crate::{ObjList, Object, Project, ProjectContext, ProjectContextMut};

/// A tiny change to the project. Used for moving backwards in time for the collaboration conflict resolution system. 
pub trait Delta {
    type Project: Project;

    fn perform(&self, context: &mut ProjectContextMut<'_, Self::Project>);
}

pub struct Recorder<'a, P: Project> {
    pub(crate) context: ProjectContextMut<'a, P>,
    /// The reversed changes recorded while the operation was being executed 
    pub(crate) deltas: Vec<Box<dyn Delta<Project = P>>>,
}

impl<'a, P: Project> Recorder<'a, P> {

    pub(crate) fn new(context: ProjectContextMut<'a, P>) -> Self {
        Self {
            context,
            deltas: Vec::new(),
        }
    }

    pub fn push_delta<D: Delta<Project = P> + 'static>(&mut self, delta: D) {
        self.deltas.push(Box::new(delta));
    }

    pub fn context(&'a self) -> ProjectContext<'a, P> {
        ProjectContext {
            project: &self.context.project,
            objects: &self.context.objects,
        }
    }

    pub fn context_mut<'b>(&'b mut self) -> &'b mut ProjectContextMut<'a, P> {
        &mut self.context
    }

    pub fn project(&self) -> &P {
        self.context.project()
    }
    
    pub fn project_mut(&mut self) -> &mut P {
        self.context.project_mut()
    }

    pub fn obj_list<O: Object<Project = P>>(&self) -> &ObjList<O> {
        self.context.obj_list()
    }

    pub fn obj_list_mut<O: Object<Project = P>>(&mut self) -> &mut ObjList<O> {
        self.context.obj_list_mut()
    }

}

