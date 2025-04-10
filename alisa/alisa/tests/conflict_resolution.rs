
mod test_server;
use test_server::TestingServer;

#[derive(Clone, alisa::Serializable, Default)]
struct Project {
    n: i32
}

#[derive(alisa::Serializable, Default)]
struct Set {
    n: i32
}

impl alisa::Operation for Set {
    type Project = Project;

    const NAME: &'static str = "Set";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Project>) -> bool {
        recorder.project_mut().n = self.n;
        true
    }

}

#[derive(alisa::Serializable, Default)]
struct Add {
    n: i32
}

impl alisa::Operation for Add {
    type Project = Project;
    const NAME: &'static str = "Add";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Project>) -> bool {
        recorder.project_mut().n += self.n;
        true
    }

}

impl alisa::Project for Project {
    type Context = ();
    type Objects = ();
    type ActionContext = ();

    fn empty() -> Self {
        Self::default()
    }

    fn create_default(&mut self) {

    }

    const OBJECTS: &'static [alisa::ObjectKind<Self>] = &[];
    const OPERATIONS: &'static [alisa::OperationKind<Self>] = &[
        alisa::OperationKind::from::<Set>(),
        alisa::OperationKind::from::<Add>(),
    ];
}

#[test]
fn basic() {
    let mut server = TestingServer::<Project>::new("basic.test");

    server.alice().queue_operation(Set { n: 50 });
    server.send_alice_messages();
    assert_eq!(server.alice().n, 50);
    assert_eq!(server.bob().n, 0);

    server.receive_bob_messages();
    assert_eq!(server.alice().n, 50);
    assert_eq!(server.bob().n, 50);

    server.bob().queue_operation(Add { n: 10 });
    server.send_bob_messages();
    assert_eq!(server.alice().n, 50);
    assert_eq!(server.bob().n, 60);

    server.receive_alice_messages();
    assert_eq!(server.alice().n, 60);
    assert_eq!(server.bob().n, 60);
    
}

#[test]
fn set_then_add() {
    let mut server = TestingServer::<Project>::new("set_then_add.test");

    server.alice().queue_operation(Set { n: 50 });
    server.bob().queue_operation(Add { n: 10 });
    server.send_alice_messages();
    server.send_bob_messages();
    assert_eq!(server.alice().n, 50);
    assert_eq!(server.bob().n, 10);

    server.receive_alice_messages();
    assert_eq!(server.alice().n, 60);

    server.receive_bob_messages();
    assert_eq!(server.bob().n, 60);
    
}

#[test]
fn add_then_set() {
    let mut server = TestingServer::<Project>::new("add_then_set.test");

    server.alice().queue_operation(Set { n: 50 });
    server.bob().queue_operation(Add { n: 10 });
    server.send_bob_messages();
    server.send_alice_messages();
    assert_eq!(server.alice().n, 50);
    assert_eq!(server.bob().n, 10);

    server.receive_alice_messages();
    assert_eq!(server.alice().n, 50);

    server.receive_bob_messages();
    assert_eq!(server.bob().n, 50);
    
}
