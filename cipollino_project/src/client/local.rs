
use std::path::PathBuf;
use crate::project::Project;
use crate::serialization::{create_project, open_project, Serializer};
use super::ProjectClient;

pub struct Local {
    pub(crate) serializer: Serializer,
    curr_key: u64
}

impl Local {

    pub fn next_key(&mut self) -> u64 {
        self.curr_key += 1;
        self.curr_key - 1
    }

    pub(crate) fn update_root_obj(&mut self, project: &Project) {
        self.serializer.update_root_project(self.curr_key, project);
    }

}

impl ProjectClient {

    pub fn local_open_project(path: PathBuf) -> Option<(ProjectClient, Project)> {
        let (serializer, curr_key, project) = open_project(path)?;
        Some((ProjectClient::Local(Local {
            serializer,
            curr_key
        }), project)) 
    }

    pub fn local_create_project(path: PathBuf, fps: f32, sample_rate: f32) -> Option<(ProjectClient, Project)> {
        let (serializer, curr_key, project) = create_project(path, fps, sample_rate)?;

        let local = Local {
            serializer, 
            curr_key,
        };

        Some((ProjectClient::Local(local), project))
    }

}
