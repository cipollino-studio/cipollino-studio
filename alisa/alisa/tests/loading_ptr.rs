
mod test_server;
use test_server::TestingServer;

#[derive(Clone, alisa::Serializable, Default)]
pub struct Project {
    node: alisa::LoadingPtr<Node>
}

alisa::project_set_property_operation!(Project, node, alisa::LoadingPtr<Node>);

#[derive(Default)]
pub struct Objects {
    nodes: alisa::ObjList<Node>
}

#[derive(Clone, alisa::Serializable, Default)]
pub struct Node {
    x: i32,
    next: alisa::LoadingPtr<Node>
}

alisa::object_set_property_operation!(Node, next, alisa::LoadingPtr<Node>);

#[derive(alisa::Serializable, Default)]
struct CreateNode {
    ptr: alisa::Ptr<Node>,
    x: i32,
    next: alisa::Ptr<Node>
}

impl alisa::Operation for CreateNode {
    type Project = Project;
    const NAME: &'static str = "CreateNode";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Project>) -> bool {
        recorder.add_obj(self.ptr, Node {
            x: self.x,
            next: alisa::LoadingPtr::new(self.next),
        })
    }
}

impl alisa::Object for Node {
    type Project = Project;

    const NAME: &'static str = "Node";
    const TYPE_ID: u16 = 0;

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.nodes
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.nodes
    }
}

impl alisa::Project for Project {

    type Objects = Objects;
    type ActionContext = ();

    fn empty() -> Self {
        Self {
            node: alisa::LoadingPtr::default()
        }
    }

    const OBJECTS: &'static [alisa::ObjectKind<Self>] = &[
        alisa::ObjectKind::from::<Node>()
    ];
    const OPERATIONS: &'static [alisa::OperationKind<Self>] = &[
        alisa::OperationKind::from::<SetNode>(),
        alisa::OperationKind::from::<CreateNode>(),
        alisa::OperationKind::from::<SetNodeNext>(),
    ];

}

#[test]
fn project_load() {

    let mut client = alisa::Client::<Project>::local("project_load.test").unwrap();
    
    let ptr = client.next_ptr().unwrap();
    client.queue_operation(CreateNode {
        ptr,
        x: 123,
        next: Default::default(),
    });
    client.queue_operation(SetNode {
        node: alisa::LoadingPtr::new(ptr),
    });
    client.tick();

    drop(client);

    let client = alisa::Client::<Project>::local("project_load.test").unwrap();
    assert!(client.get(ptr).is_some());
    assert_eq!(client.get(ptr).unwrap().x, 123);

    let _ = std::fs::remove_file("project_load.test");

}

#[test]
fn object_load() {

    let mut client = alisa::Client::<Project>::local("object_load.test").unwrap();

    let cdr = client.next_ptr().unwrap();
    client.queue_operation(CreateNode {
        ptr: cdr,
        x: 2,
        next: alisa::Ptr::null(),
    });
    let car = client.next_ptr().unwrap();
    client.queue_operation(CreateNode {
        ptr: car,
        x: 1,
        next: cdr,
    });

    client.tick();

    drop(client);

    let mut client = alisa::Client::<Project>::local("object_load.test").unwrap();
    client.request_load(car);
    client.tick();
    assert!(client.get(car).is_some());
    assert_eq!(client.get(car).unwrap().x, 1);
    assert!(client.get(cdr).is_some());
    assert_eq!(client.get(cdr).unwrap().x, 2);

    let _ = std::fs::remove_file("object_load.test");

}

#[test]
fn project_load_collab() {

    let mut server = TestingServer::<Project>::new("project_load_collab.test"); 

    // Make sure Alice gets keys
    server.tick_alice();
    server.stabilize();

    let ptr = server.alice().next_ptr().unwrap();
    server.alice().queue_operation(CreateNode {
        ptr,
        x: 123,
        next: Default::default(),
    });
    server.alice().queue_operation(SetNode {
        node: alisa::LoadingPtr::new(ptr),
    });
    server.tick_alice();
    server.stabilize();

    let carol = server.add_client();
    assert!(server.client(carol).get(ptr).is_some());
    assert_eq!(server.client(carol).get(ptr).unwrap().x, 123);
    
}

#[test]
fn object_load_collab() {

    let mut server = TestingServer::<Project>::new("project_load_collab.test"); 

    // Make sure Alice gets keys
    server.tick_alice();
    server.stabilize();

    let cdr = server.alice().next_ptr().unwrap();
    server.alice().queue_operation(CreateNode {
        ptr: cdr,
        x: 2,
        next: alisa::Ptr::null(),
    });
    let car = server.alice().next_ptr().unwrap();
    server.alice().queue_operation(CreateNode {
        ptr: car,
        x: 1,
        next: cdr,
    });

    server.tick_alice();
    server.stabilize();

    let carol = server.add_client();
    server.client(carol).request_load(car);
    server.tick_client(carol);
    server.stabilize();
    assert!(server.client(carol).get(cdr).is_some());
    assert_eq!(server.client(carol).get(cdr).unwrap().x, 2);
    
}
