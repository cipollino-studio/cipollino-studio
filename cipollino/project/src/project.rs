
use crate::{CreateFolder, DeleteFolder, Folder};

#[derive(alisa::Serializable)]
#[project(Project)]
pub struct Project {
    pub folders: alisa::UnorderedChildList<Folder>
}

impl Default for Project {

    fn default() -> Self {
        Self {
            folders: Default::default()
        }
    }

}

#[derive(Default)]
pub struct Objects { 
    pub folders: alisa::ObjList<Folder>
}

impl alisa::Project for Project {

    type Context = ();
    type Objects = Objects;

    fn empty() -> Self {
        Self::default()
    }

    fn create_default(&mut self) {

    }

    fn verter_config() -> verter::Config {
        verter::Config {
            magic_bytes: b"CIPOLINO",
            page_size: 64,
        } 
    }

    const OBJECTS: &'static [alisa::ObjectKind<Self>] = &[
        alisa::ObjectKind::from::<Folder>()
    ];

    const OPERATIONS: &'static [alisa::OperationKind<Self>] = &[
        alisa::OperationKind::from::<CreateFolder>(),
        alisa::OperationKind::from::<DeleteFolder>(),
    ];

}