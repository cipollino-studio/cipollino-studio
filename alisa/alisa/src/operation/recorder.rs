use std::{cell::RefCell, collections::HashSet};

use crate::{AnyPtr, ObjRef, Object, Project, ProjectContext, ProjectContextMut, Ptr};

use super::{Delta, OperationSource};

pub struct Recorder<'a, P: Project> {
    pub(crate) context: ProjectContextMut<'a, P>,
    /// Where the operation in question originated
    pub(crate) source: OperationSource,

    /// The reversed changes recorded while the operation was being executed 
    pub(crate) delta: Option<&'a mut Delta<P>>,

    /// What objects were already modified by this recorder
    modified: HashSet<AnyPtr>,
    /// Was the project modified by the recorder
    modified_project: bool,

    /// Was the operation successful?
    /// Note: the operation's `perform` method could also indicate that the operation was unsuccessful
    pub(crate) success: RefCell<bool>
}

impl<'a, P: Project> Recorder<'a, P> {

    pub(crate) fn new(context: ProjectContextMut<'a, P>, source: OperationSource, delta: Option<&'a mut Delta<P>>) -> Self {
        Self {
            context,
            delta,
            source,
            modified: HashSet::new(),
            modified_project: false,
            success: RefCell::new(true)
        }
    }

    pub fn context(&'a self) -> ProjectContext<'a, P> {
        ProjectContext {
            project: &self.context.project,
            objects: &self.context.objects,
        }
    }

    pub fn project(&self) -> &P {
        self.context.project()
    }

    pub fn project_mut(&mut self) -> &mut P {
        if let Some(delta) = self.delta.as_mut() {
            if !self.modified_project {
                self.modified_project = true;
                let old_project = self.context.project.clone();
                delta.push(move |context| {
                    *context.project_mut() = old_project.clone();
                });
            }
        }
        self.context.project_mut()
    }

    pub fn get_obj<O: Object<Project = P>>(&self, ptr: Ptr<O>) -> Option<&O> {
        match self.context.obj_list().get_ref(ptr) {
            ObjRef::None | ObjRef::Loading => {
                *self.success.borrow_mut() = false;
                None
            },
            ObjRef::Loaded(obj) => Some(obj),
            ObjRef::Deleted => None,
        }
    }

    pub fn get_obj_mut<O: Object<Project = P>>(&mut self, ptr: Ptr<O>) -> Option<&mut O> {

        match self.context.obj_list().get_ref(ptr) {
            ObjRef::None | ObjRef::Loading => {
                *self.success.borrow_mut() = false;
                return None; 
            },
            ObjRef::Loaded(_) => {},
            ObjRef::Deleted => { return None; }
        }

        let object = self.context.obj_list_mut().get_mut(ptr)?;
        if let Some(delta) = &mut self.delta {
            if self.modified.insert(ptr.any()) {
                let old_object = object.clone();
                delta.push(move |context| {
                    if let Some(obj) = context.obj_list_mut().get_mut(ptr) {
                        *obj = old_object.clone();
                    }
                });
            }
        }
        Some(object)
    }
    
    /// Add an object to the project. Returns true if successful.
    pub fn add_obj<O: Object<Project = P>>(&mut self, ptr: Ptr<O>, object: O) -> bool {
        if self.context.obj_list().get(ptr).is_some() {
            return false;
        }
        if let Some(delta) = &mut self.delta {
            delta.push(move |context| {
                context.obj_list_mut().delete(ptr);
            });
        }
        self.context.obj_list_mut().insert(ptr, object);
        self.modified.insert(ptr.any());
        true
    }

    pub fn delete_obj<O: Object<Project = P>>(&mut self, ptr: Ptr<O>) -> Option<O> {
        let object = self.context.obj_list_mut().delete(ptr)?;
        if let Some(delta) = &mut self.delta {
            let object_copy = object.clone();
            delta.push(move |context| {
                context.obj_list_mut().insert(ptr, object_copy.clone());
            });
            self.modified.insert(ptr.any());
        }
        Some(object)
    }

    pub fn source(&self) -> OperationSource {
        self.source
    }

}
