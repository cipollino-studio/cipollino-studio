use std::path::Path;

use keymap::Keymap;

use crate::{rmpv_decode, rmpv_encode, rmpv_get, DeserializationContext, Project, SerializationContext};

mod keymap;

pub(crate) struct File {
    /// The Verter file. Verter is used to allow O(1) incremental file reads/updates. For more info, see [Verter on crates.io](https://crates.io/crates/verter).
    file: verter::File,
    /// The pointer to the project data in the Verter file.
    project_ptr: u64,
    keymap: Keymap
}

impl File {

    fn try_open(file: &mut verter::File) -> Option<(Keymap, u64, u64)> {

        // Load file metadata
        let root_data = file.read_root().ok()?;
        let mut root_data = root_data.as_slice();
        let root_data = rmpv::decode::read_value(&mut root_data).ok()?;
        let curr_key = rmpv_get(&root_data, "curr_key")?.as_u64()?;
        let project_ptr = rmpv_get(&root_data, "project_ptr")?.as_u64()?;
        let keymap_ptr = rmpv_get(&root_data, "keymap_ptr")?.as_u64()?;

        // Initialize the keymap
        let keymap = Keymap::new(keymap_ptr); 

        Some((keymap, curr_key, project_ptr))
    }

    fn try_load_project<P: Project>(&mut self) -> Option<(P, P::Objects)> {
        let project_data = self.read(self.project_ptr)?;
        let mut objects = P::Objects::default();
        let mut context = DeserializationContext::local(&mut objects, self);
        let project = P::deserialize(&project_data, &mut context)?; 
        Some((project, objects))
    }

    fn write_root(file: &mut verter::File, curr_key: u64, project_ptr: u64, keymap_ptr: u64) {
        if let Some(data) = rmpv_encode(&rmpv::Value::Map(vec![
            ("curr_key".into(), curr_key.into()),
            ("project_ptr".into(), project_ptr.into()),
            ("keymap_ptr".into(), keymap_ptr.into()),
        ])) {
            let _ = file.write_root(&data);
        }
    }

    pub fn open<P: Project, PathRef: AsRef<Path>>(path: PathRef) -> Option<(Self, P, P::Objects, u64)> {

        let mut file = verter::File::open(path, P::verter_config()).ok()?; // TODO: add configuration for magic bytes

        // Load the project

        let (keymap, curr_key, project_ptr) = if let Some((keymap, curr_key, project_ptr)) = Self::try_open(&mut file) {
            (keymap, curr_key, project_ptr)
        } else {
            let curr_key = 1;
            let (keymap, keymap_ptr) = Keymap::create_empty(&mut file)?;
            let project_ptr = file.alloc().ok()?; 

            Self::write_root(&mut file, curr_key, project_ptr, keymap_ptr);

            (keymap, curr_key, project_ptr)
        };

        let mut file = Self {
            file,
            project_ptr,
            keymap,
        };

        if file.read_bytes(file.project_ptr).is_none() {
            file.project_ptr = file.file.alloc().ok()?;
        }

        let (project, objects) = if let Some((projects, objects)) = file.try_load_project() {
            (projects, objects)
        } else {
            let project = P::empty();
            let objects = P::Objects::default();

            let project_data = project.serialize(&SerializationContext::shallow());
            file.write_project(&project_data);

            (project, objects)
        };

        Some((file, project, objects, curr_key)) 
    }

    pub fn read_bytes(&mut self, ptr: u64) -> Option<Vec<u8>> {
        self.file.read(ptr).ok()
    }

    pub fn read(&mut self, ptr: u64) -> Option<rmpv::Value> {
        rmpv_decode(&self.read_bytes(ptr)?)
    }

    pub fn write_bytes(&mut self, ptr: u64, data: &[u8]) {
        let _ = self.file.write(ptr, data);
    }

    pub fn write(&mut self, ptr: u64, data: &rmpv::Value) {
        if let Some(data) = rmpv_encode(data) {
            self.write_bytes(ptr, &data);
        }
    }

    pub fn write_project(&mut self, data: &rmpv::Value) { 
        self.write(self.project_ptr, data);
    }

    pub fn update_root(&mut self, curr_key: u64) {
        Self::write_root(&mut self.file, curr_key, self.project_ptr, self.keymap.ptr());
    }

    pub fn get_ptr(&mut self, key: u64) -> Option<u64> {
        self.keymap.get_ptr(key, &mut self.file)
    }

    pub fn delete(&mut self, key: u64) {
        self.keymap.delete(key, &mut self.file);
    }

}
