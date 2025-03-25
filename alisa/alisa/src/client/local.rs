
use std::{cell::RefCell, marker::PhantomData, path::Path};

use crate::{File, ObjectKind, Project, SerializationContext};

use super::{Client, ClientKind};

#[cfg(debug_assertions)]
use super::verify_project_type;

pub(crate) struct Local<P: Project> {
    /// The Verter file to which the project is saved
    file: File,
    /// The next key available for use
    curr_key: RefCell<u64>,
    /// Does the root data of the Verter file need to be updated?
    root_data_modified: RefCell<bool>,

    /// Marker to make sure the type `P`` is used
    _marker: PhantomData<P>
}

impl<P: Project> Local<P> {

    pub(crate) fn new(file: File, curr_key: u64) -> Self {
        Self {
            file,
            curr_key: RefCell::new(curr_key),
            root_data_modified: RefCell::new(false),
            _marker: PhantomData
        }
    }

    pub(crate) fn next_key(&self) -> u64 {
        let mut curr_key = self.curr_key.borrow_mut();
        let key = *curr_key;
        *curr_key += 1;
        *self.root_data_modified.borrow_mut() = true;
        key
    }

    pub(crate) fn next_key_range(&self, n_keys: u64) -> (u64, u64) {
        let mut curr_key = self.curr_key.borrow_mut();
        let first = *curr_key;
        *curr_key += n_keys;
        *self.root_data_modified.borrow_mut() = true;
        (first, first + n_keys - 1)
    }

    fn update_root_data(&mut self) {
        self.file.update_root(*self.curr_key.borrow()); 
    }

    pub(crate) fn save_changes(&mut self, project: &mut P, objects: &mut P::Objects, project_modified: &mut bool) {

        // Update file root data if necessary 
        if *self.root_data_modified.borrow() {
            self.update_root_data();
            *self.root_data_modified.borrow_mut() = false;
        }

        // Project modifications
        if *project_modified {
            let data = project.serialize(&SerializationContext::shallow());
            self.file.write_project(&data);
            *project_modified = false;
        }

        // Object modifications
        for object_kind in P::OBJECTS {
            (object_kind.save_modifications)(&mut self.file, objects);
        }

    }

    pub(crate) fn load_objects(&mut self, objects: &mut P::Objects) {
        for object_kind in P::OBJECTS {
            (object_kind.load_objects)(&mut self.file, objects)
        }
    }

    pub(crate) fn dyn_load(&mut self, obj_kind: &ObjectKind<P>, objects: &mut P::Objects, key: u64) {
        (obj_kind.load_object)(&mut self.file, objects, key);
    }

}

impl<P: Project> Client<P> {

    pub fn local<PathRef: AsRef<Path>>(path: PathRef) -> Option<Self> {

        #[cfg(debug_assertions)]
        verify_project_type::<P>();
        
        let (file, project, objects, curr_key) = File::open(path)?;

        Some(Self {
            kind: ClientKind::Local(Local::new(file, curr_key)),
            project,
            objects,
            operations_to_perform: RefCell::new(Vec::new()),
            project_modified: false,
            undo_stack: RefCell::new(Vec::new()),
            redo_stack: RefCell::new(Vec::new())
        })
    }

}
