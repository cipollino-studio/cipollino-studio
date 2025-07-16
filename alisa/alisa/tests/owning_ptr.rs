
#[derive(Clone, alisa::Serializable, Default)]
pub struct Project {
    node: alisa::LoadingPtr<Node>
}

#[derive(Default)]
pub struct Objects {
    nodes: alisa::ObjList<Node>
}

#[derive(Clone, alisa::Serializable, Default)]
pub struct Node {
    x: i32,
    next: alisa::OwningPtr<Node>
}

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
            next: alisa::OwningPtr::new(self.next),
        })
    }
}

#[derive(alisa::Serializable, Default)]
struct DeleteNode {
    ptr: alisa::Ptr<Node>,
}

impl alisa::Operation for DeleteNode {
    type Project = Project;
    const NAME: &'static str = "DeleteNode";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Project>) -> bool {
        recorder.delete_obj(self.ptr);
        true
    }
}

impl alisa::Object for Node {
    type Project = Project;

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
        alisa::OperationKind::from::<CreateNode>(),
        alisa::OperationKind::from::<DeleteNode>(),
    ];

}

#[test]
fn owning_ptr() {

    let mut client = alisa::Client::<Project>::local("owning_ptr.test").unwrap();

    let cdr = client.next_ptr();
    client.queue_operation(CreateNode {
        ptr: cdr,
        x: 2,
        next: alisa::Ptr::null(),
    });
    let car = client.next_ptr();
    client.queue_operation(CreateNode {
        ptr: car,
        x: 1,
        next: cdr,
    });
    client.tick();

    client.queue_operation(DeleteNode {
        ptr: car
    });
    client.tick();

    assert!(client.get_ref(car).is_deleted());
    assert!(client.get_ref(cdr).is_deleted());

    let _ = std::fs::remove_file("owning_ptr.test");

}
