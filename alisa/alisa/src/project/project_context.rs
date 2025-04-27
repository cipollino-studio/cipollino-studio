
use crate::{ObjList, Object};
use super::Project;

pub struct ProjectContext<'a, P: Project> {
    pub(crate) project: &'a P,
    pub(crate) objects: &'a P::Objects,
}

impl<P: Project> ProjectContext<'_, P> {

    pub fn project(&self) -> &P {
        self.project
    }

    pub fn objects(&self) -> &P::Objects {
        self.objects
    }

    pub fn obj_list<O: Object<Project = P>>(&self) -> &ObjList<O> {
        O::list(self.objects)
    }

}

pub struct ProjectContextMut<'a, P: Project> {
    pub(crate) project: &'a mut P,
    pub(crate) objects: &'a mut P::Objects,

    /// Was the project object itself modified?
    pub(crate) project_modified: &'a mut bool
}

impl<P: Project> ProjectContextMut<'_, P> {

    pub fn project(&self) -> &P {
        self.project
    }
    
    pub fn project_mut(&mut self) -> &mut P {
        *self.project_modified = true;
        self.project
    }

    pub fn objects(&self) -> &P::Objects {
        &self.objects
    }

    pub fn objects_mut(&mut self) -> &mut P::Objects {
        self.objects
    }

    pub fn obj_list<O: Object<Project = P>>(&self) -> &ObjList<O> {
        O::list(self.objects)
    }

    pub fn obj_list_mut<O: Object<Project = P>>(&mut self) -> &mut ObjList<O> {
        O::list_mut(self.objects)
    }

}
