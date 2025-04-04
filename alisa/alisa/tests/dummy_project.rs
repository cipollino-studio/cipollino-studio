
use alisa::Project;

#[derive(Default, alisa::Serializable, Clone)]
pub struct DummyProject {

}

impl Project for DummyProject {
    type Context = ();
    type Objects = ();
    type ActionContext = ();

    fn empty() -> Self {
        Self {}
    }

    fn create_default(&mut self) {

    }

    const OBJECTS: &'static [alisa::ObjectKind<Self>] = &[];
    const OPERATIONS: &'static [alisa::OperationKind<Self>] = &[];

} 
