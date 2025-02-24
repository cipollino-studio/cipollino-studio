
use alisa::Project;

#[derive(Default, alisa::Serializable)]
pub struct DummyProject {

}

impl Project for DummyProject {
    type Context = ();
    type Objects = ();

    fn empty() -> Self {
        Self {}
    }

    fn create_default(&mut self) {

    }

    const OBJECTS: &'static [alisa::ObjectKind<Self>] = &[];
    const OPERATIONS: &'static [alisa::OperationKind<Self>] = &[];

} 
