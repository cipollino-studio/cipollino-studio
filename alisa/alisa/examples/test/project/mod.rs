
use folder::{CreateFolder, DeleteFolder, Folder, SetFolderName, TransferFolder};

pub mod folder;

#[derive(Default)]
pub struct ProjectObjects {
    pub folders: alisa::ObjList<Folder>,
}

#[derive(alisa::Serializable)]
#[project(Project)]
pub struct Project {
    pub n: i32,
    pub folders: alisa::UnorderedChildList<Folder> 
}

impl Default for Project {
    fn default() -> Self {
        Self {
            n: 0,
            folders: alisa::UnorderedChildList::default() 
        }
    }
}

impl alisa::Project for Project {
    type Context = ();
    type Objects = ProjectObjects;

    fn empty() -> Self {
        Self {
            n: 0,
            folders: alisa::UnorderedChildList::default() 
        }
    }

    fn create_default(&mut self) {

    }

    const OBJECTS: &'static [alisa::ObjectKind<Self>] = &[
        alisa::ObjectKind::from::<Folder>()
    ];

    const OPERATIONS: &'static [alisa::OperationKind<Self>] = &[
        alisa::OperationKind::from::<SetN>(),
        alisa::OperationKind::from::<IncrN>(),
        alisa::OperationKind::from::<DecrN>(),

        alisa::OperationKind::from::<CreateFolder>(),
        alisa::OperationKind::from::<DeleteFolder>(),
        alisa::OperationKind::from::<TransferFolder>(),
        alisa::OperationKind::from::<SetFolderName>(),
    ];
}

alisa::project_set_property_operation!(Project, n, i32);

#[derive(alisa::Serializable, Default)]
pub struct IncrN;

#[derive(alisa::Serializable, Default)]
pub struct DecrN;

impl alisa::Operation for IncrN {
    type Project = Project;

    type Inverse = DecrN;

    const NAME: &'static str = "IncrN";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Self::Project>) -> bool {
        recorder.push_delta(SetNDelta {
            n: recorder.project().n,
        });
        recorder.project_mut().n += 1;
        true
    }

    fn inverse(&self, _context: &alisa::ProjectContext<Project>) -> Option<Self::Inverse> {
        Some(DecrN)
    }
}

impl alisa::Operation for DecrN {
    type Project = Project;

    type Inverse = IncrN;

    const NAME: &'static str = "DecrN";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Self::Project>) -> bool {
        recorder.push_delta(SetNDelta {
            n: recorder.project().n,
        });
        recorder.project_mut().n -= 1;
        true
    }

    fn inverse(&self, _context: &alisa::ProjectContext<Project>) -> Option<Self::Inverse> {
        Some(IncrN)
    }
}
