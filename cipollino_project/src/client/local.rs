
use std::collections::HashMap;
use std::path::PathBuf;

use crate::project::folder::Folder;
use crate::project::obj::{Obj, ObjPtr};
use crate::{project::Project, serialization::project_file::ProjectFile};
use crate::serialization::{ObjSerialize, Serializer};
use super::ProjectClient;

pub struct Local {
    serializer: Serializer,
    curr_key: u64
}

impl Local {

    pub fn next_key(&mut self) -> u64 {
        self.curr_key += 1;
        self.curr_key - 1
    }

    pub(crate) fn update_root_obj(&mut self, project: &Project) {
        let _ = self.serializer.project_file.set_root_obj(bson::bson!({
            "curr_key": self.curr_key as i64,
            "root_folder_key": project.root_folder.key as i64,
            "root_folder_ptr": *self.serializer.obj_ptrs.get(&project.root_folder.key).unwrap() as i64,
            "fps": project.fps,
            "sample_rate": project.sample_rate
        }));
    }

    pub(crate) fn set_obj_data<T: Obj>(&mut self, project: &Project, ptr: ObjPtr<T>) {
        let Some(obj) = T::obj_list(project).get(ptr) else { return; };
        let data = obj.obj_serialize(project, &mut self.serializer);
        let Some(ptr) = self.serializer.get_ptr(ptr.key) else { return; };
        let _ = self.serializer.project_file.set_obj_data(ptr, data);
    }

}

impl ProjectClient {

    pub fn local_open_project(path: PathBuf) -> Option<(ProjectClient, Project)> {
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

        let root_folder = Folder::obj_deserialize(&mut project, &serializer.project_file.get_obj_data(root_folder_ptr).ok()?, &mut serializer)?;
        project.folders.objs.insert(ObjPtr::from_key(root_folder_key), root_folder);
        project.root_folder = ObjPtr::from_key(root_folder_key);
        
        Some((ProjectClient::Local(Local {
            serializer,
            curr_key
        }), project)) 
    }

    pub fn local_create_project(path: PathBuf, fps: f32, sample_rate: f32) -> Option<(ProjectClient, Project)> {
        let project = Project::new(fps, sample_rate);
        let mut project_file = ProjectFile::create(&path, bson::bson!({})).ok()?;
        let root_folder_file_ptr = project_file.alloc_page().ok()?;

        let mut serializer = Serializer {
            project_file: project_file,
            obj_ptrs: HashMap::from_iter([(project.root_folder.key, root_folder_file_ptr)]),
        };

        let root_folder_data = project.root_folder().obj_serialize(&project, &mut serializer);
        serializer.project_file.set_obj_data(root_folder_file_ptr, root_folder_data).ok()?;

        let mut local = Local {
            serializer, 
            curr_key: 2,
        };

        local.update_root_obj(&project);

        Some((ProjectClient::Local(local), project))
    }

}
