
use std::{collections::HashMap, path::PathBuf};

use project_file::ProjectFile;

use crate::{crdt::{fractional_index::FractionalIndex, register::Register}, project::{folder::Folder, obj::{Obj, ObjPtr, ObjState}, Project}};

pub mod project_file;

pub struct Serializer {
    pub(crate) obj_ptrs: HashMap<u64, u64>,
    pub(crate) project_file: ProjectFile
}

impl Serializer {

    pub(crate) fn get_ptr(&mut self, key: u64) -> Option<u64> {
        if let Some(ptr) = self.obj_ptrs.get(&key) {
            Some(*ptr)
        } else {
            let ptr = self.project_file.alloc_page().ok()?;
            self.obj_ptrs.insert(key, ptr);
            Some(ptr)
        }
    }

    pub(crate) fn update_root_project(&mut self, curr_key: u64, project: &Project) {
        let _ = self.project_file.set_root_obj(bson::bson!({
            "curr_key": curr_key as i64, 
            "root_folder_key": project.root_folder.key as i64,
            "root_folder_ptr": *self.obj_ptrs.get(&project.root_folder.key).unwrap() as i64,
            "fps": project.fps,
            "sample_rate": project.sample_rate
        }));
    }

    pub(crate) fn set_obj_data<T: Obj>(&mut self, project: &Project, ptr: ObjPtr<T>) {
        let Some(obj) = T::obj_list(project).get(ptr) else { return; };
        let data = obj.obj_serialize(project, self);
        let Some(ptr) = self.get_ptr(ptr.key) else { return; };
        let _ = self.project_file.set_obj_data(ptr, data);
    }

}

pub trait ObjSerialize: Sized {

    fn obj_serialize(&self, project: &Project, serializer: &mut Serializer) -> bson::Bson; 
    fn obj_deserialize(project: &mut Project, data: &bson::Bson, serializer: &mut Serializer, idx: FractionalIndex) -> Option<Self>;

}

impl ObjSerialize for bool {

    fn obj_serialize(&self, _project: &Project, _serializer: &mut Serializer) -> bson::Bson {
        bson::Bson::Boolean(*self)
    }

    fn obj_deserialize(_project: &mut Project, data: &bson::Bson, _serializer: &mut Serializer, _idx: FractionalIndex) -> Option<Self> {
        Some(data.as_bool()?)
    }

}

impl ObjSerialize for i32 {

    fn obj_serialize(&self, _project: &Project, _serializer: &mut Serializer) -> bson::Bson {
        bson::Bson::Int32(*self)
    }

    fn obj_deserialize(_project: &mut Project, data: &bson::Bson, _serializer: &mut Serializer, _idx: FractionalIndex) -> Option<Self> {
        Some(data.as_i32()?)
    }

}

impl ObjSerialize for f32 {

    fn obj_serialize(&self, _project: &Project, _serializer: &mut Serializer) -> bson::Bson {
        bson::Bson::Double(*self as f64)
    }

    fn obj_deserialize(_project: &mut Project, data: &bson::Bson, _serializer: &mut Serializer, _idx: FractionalIndex) -> Option<Self> {
        Some(data.as_f64()? as f32)
    }

}

impl ObjSerialize for String {

    fn obj_serialize(&self, _project: &Project, _serializer: &mut Serializer) -> bson::Bson {
        bson::Bson::String(self.clone())
    }

    fn obj_deserialize(_project: &mut Project, data: &bson::Bson, _serializer: &mut Serializer, _idx: FractionalIndex) -> Option<Self> {
        Some(data.as_str()?.to_owned()) 
    }

}

impl<T: ObjSerialize> ObjSerialize for Register<T> {

    fn obj_serialize(&self, project: &Project, serialization: &mut Serializer) -> bson::Bson {
        self.value.obj_serialize(project, serialization)
    }

    fn obj_deserialize(project: &mut Project, data: &bson::Bson, serializer: &mut Serializer, idx: FractionalIndex) -> Option<Self> {
        Some(Self::new(T::obj_deserialize(project, data, serializer, idx)?, 0))
    }

}

pub fn create_project(path: PathBuf, fps: f32, sample_rate: f32) -> Option<(Serializer, u64, Project)> {
    let project = Project::new(fps, sample_rate);
    let mut project_file = ProjectFile::create(&path, bson::bson!({})).ok()?;
    let root_folder_file_ptr = project_file.alloc_page().ok()?;

    let mut serializer = Serializer {
        project_file: project_file,
        obj_ptrs: HashMap::from_iter([(project.root_folder.key, root_folder_file_ptr)]),
    };

    let root_folder_data = project.root_folder().obj_serialize(&project, &mut serializer);
    serializer.project_file.set_obj_data(root_folder_file_ptr, root_folder_data).ok()?;

    serializer.update_root_project(2, &project);

    Some((serializer, 2, project))
}

pub fn open_project(path: PathBuf) -> Option<(Serializer, u64, Project)> {
     let mut project_file = ProjectFile::open(&path).ok()?;
    let root_obj = project_file.get_root_obj().ok()?;
    let root_obj = root_obj.as_document()?;

    let curr_key = root_obj.get("curr_key")?.as_i64()?;
    if curr_key <= 0 {
        return None;
    }
    let curr_key = curr_key as u64;

    let root_folder_key = root_obj.get("root_folder_key")?.as_i64()?;
    if root_folder_key <= 0 {
        return None;
    }
    let root_folder_key = root_folder_key as u64;

    let root_folder_ptr = root_obj.get("root_folder_ptr")?.as_i64()?;
    if root_folder_ptr <= 0 {
        return None;
    }
    let root_folder_ptr = root_folder_ptr as u64;

    let mut project = Project::new(
        root_obj.get("fps").map(|fps| fps.as_f64()).flatten().unwrap_or(24.0) as f32,
        root_obj.get("sample_rate").map(|sample_rate| sample_rate.as_f64()).flatten().unwrap_or(44100.0) as f32,
    ); 

    let mut serializer = Serializer {
        obj_ptrs: HashMap::from_iter([(root_folder_key, root_folder_ptr)]),
        project_file,
    };

    let root_folder = Folder::obj_deserialize(&mut project, &serializer.project_file.get_obj_data(root_folder_ptr).ok()?, &mut serializer, FractionalIndex::half())?;
    project.folders.objs.insert(ObjPtr::from_key(root_folder_key), ObjState::Loaded(root_folder));
    project.root_folder = ObjPtr::from_key(root_folder_key);

    Some((serializer, curr_key, project))
}
