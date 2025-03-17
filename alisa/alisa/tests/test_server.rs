
use std::path::PathBuf;

struct TestingClient<P: alisa::Project> {
    id: alisa::ClientId,
    client: alisa::Client<P> 
}

impl<P: alisa::Project> TestingClient<P> {
    
    fn new(server: &mut alisa::Server<P>) -> Self {
        let (id, welcome_data) = server.add_client();
        Self {
            client: alisa::Client::collab(&welcome_data).unwrap(),
            id
        }
    }

}

pub struct TestingServer<P: alisa::Project> {
    server: alisa::Server<P>,

    alice: TestingClient<P>,
    bob: TestingClient<P>, 

    path: PathBuf
}

impl<P: alisa::Project<Context = ()>> TestingServer<P> {

    pub fn new<Path: Into<PathBuf>>(path: Path) -> Self {
        let path = path.into();
        let mut server = alisa::Server::new(path.clone(), ()).unwrap();

        let alice = TestingClient::new(&mut server);
        let bob = TestingClient::new(&mut server);

        let mut server = Self {
            server,
            alice,
            bob,
            path
        };

        server.stabilize();

        server
    }

    fn is_client_stable(&self, client: &TestingClient<P>) -> bool {
        !client.client.has_messages() && self.server.get_msgs_to_send(client.id).map(|msgs| msgs.is_empty()).unwrap_or(true)
    }

    pub fn is_stable(&self) -> bool {
        self.is_client_stable(&self.alice) && self.is_client_stable(&self.bob)
    }

    fn send_messages(server: &mut alisa::Server<P>, client: &TestingClient<P>) {
        let messages = client.client.take_messages();
        for message in messages {
            server.receive_message(client.id, message);
        }
    }

    fn receive_messages(server: &mut alisa::Server<P>, client: &mut TestingClient<P>) {
        if let Some(messages) = server.get_msgs_to_send_mut(client.id) {
            let messages = std::mem::replace(messages, Vec::new());
            for message in messages {
                client.client.receive_message(message, &mut ());
            }
        }
    }

    pub fn alice(&self) -> &alisa::Client<P> {
        &self.alice.client
    }

    pub fn bob(&self) -> &alisa::Client<P> {
        &self.bob.client
    }

    pub fn tick_alice(&mut self) {
        self.alice.client.tick(&mut ());
    }

    pub fn tick_bob(&mut self) {
        self.bob.client.tick(&mut ());
    }

    pub fn send_alice_messages(&mut self) {
        self.tick_alice();
        Self::send_messages(&mut self.server, &self.alice);
    }

    pub fn receive_alice_messages(&mut self) {
        Self::receive_messages(&mut self.server, &mut self.alice);
    }

    pub fn send_bob_messages(&mut self) {
        self.tick_bob();
        Self::send_messages(&mut self.server, &self.bob);
    }

    pub fn receive_bob_messages(&mut self) {
        Self::receive_messages(&mut self.server, &mut self.bob);
    }

    pub fn stabilize(&mut self) {
        while !self.is_stable() {
            self.send_alice_messages();
            self.send_bob_messages();
            self.receive_alice_messages();
            self.receive_bob_messages();
        }
    }

}

impl<P: alisa::Project> Drop for TestingServer<P> {

    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }

}
