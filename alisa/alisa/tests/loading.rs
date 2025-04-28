
mod test_server;

use test_server::TestingServer;

#[derive(Clone, alisa::Serializable, Default)]
pub struct Project {

}

#[derive(Default)]
pub struct Objects {
    things: alisa::ObjList<Thing>
}

#[derive(Clone, alisa::Serializable, Default)]
pub struct Thing {
    x: i32
}

impl alisa::Object for Thing {

    type Project = Project;
    const TYPE_ID: u16 = 0;

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.things
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.things
    }
}

alisa::object_set_property_operation!(Thing, x, i32);

#[derive(alisa::Serializable, Default)]
pub struct CreateThing {
    ptr: alisa::Ptr<Thing>,
    x: i32
}

impl alisa::Operation for CreateThing {

    type Project = Project;
    const NAME: &'static str = "CreateThing";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Self::Project>) -> bool {
        recorder.add_obj(self.ptr, Thing {
            x: self.x,
        })
    }
}

impl alisa::Project for Project {

    type Objects = Objects;
    type ActionContext = ();

    fn empty() -> Self {
        Self {}
    }

    const OBJECTS: &'static [alisa::ObjectKind<Self>] = &[
        alisa::ObjectKind::from::<Thing>()
    ];
    const OPERATIONS: &'static [alisa::OperationKind<Self>] = &[
        alisa::OperationKind::from::<CreateThing>(),
        alisa::OperationKind::from::<SetThingX>()
    ];
}

#[test]
fn basic_loading() {

    let mut server = TestingServer::<Project>::new("basic_loading.test");

    // Make sure Alice gets some keys in her keychain
    server.tick_alice();
    server.stabilize();

    let ptr = server.alice().next_ptr();
    let next_ptr = server.alice().next_ptr();
    server.alice().queue_operation(CreateThing {
        ptr,
        x: 123,
    });

    server.stabilize();

    // Introduce a new client who hasn't loaded Alice's thing 
    let carol = server.add_client();

    assert!(server.client(carol).get(ptr).is_none()); // Carol doesn't have the thing...
    assert!(server.client(carol).get_ref(ptr).is_none()); // Never heard of it, even.

    // Request load
    server.client(carol).request_load(ptr);
    server.tick_client(carol);

    assert!(server.client(carol).get(ptr).is_none()); // Still doesn't know about it...
    assert!(server.client(carol).get_ref(ptr).is_loading()); // But she's gonna load it!

    server.stabilize();
    assert!(server.client(carol).get(ptr).is_some_and(|thing| thing.x == 123)); // Aha! Got it!
    assert!(server.client(carol).get_ref(ptr).is_loaded());

    // Now, Carol hears rumors of another mysterious thing...
    assert!(server.alice().get(next_ptr).is_none()); // Alice(who's been here since the start) never heard of it.

    // But Carol is undeterred.

    // She seeks out the mysterious thing 
    server.client(carol).request_load(next_ptr);
    server.tick_client(carol);

    assert!(server.client(carol).get(next_ptr).is_none()); // Still doesn't have the answers...
    assert!(server.client(carol).get_ref(next_ptr).is_loading()); // But surely the thing must exist!

    server.stabilize();
    assert!(server.client(carol).get(next_ptr).is_none()); // Alas, it seems the rumors were false. 
    assert!(server.client(carol).get_ref(next_ptr).is_deleted()); // Even the all-knowing oracle(the server) tells her it doesn't exist.

    // But one day... the object does actually get created!
    server.alice().queue_operation(CreateThing {
        ptr: next_ptr,
        x: 456,
    });
    server.stabilize();

    // Carol tries to load it again...
    server.client(carol).request_load(next_ptr);
    server.tick_client(carol);
    assert!(server.client(carol).get(next_ptr).is_none());
    assert!(server.client(carol).get_ref(next_ptr).is_loading());
    
    // And finds it!
    server.stabilize();
    assert!(server.client(carol).get(next_ptr).is_some_and(|thing| thing.x == 456));
    assert!(server.client(carol).get_ref(next_ptr).is_loaded());

}
