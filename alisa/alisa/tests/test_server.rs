
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

    clients: Vec<TestingClient<P>>,

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
            clients: vec![alice, bob],
            path
        };

        server.stabilize();

        server
    }

    fn is_client_stable(&self, client: &TestingClient<P>) -> bool {
        !client.client.has_messages() && self.server.get_msgs_to_send(client.id).map(|msgs| msgs.is_empty()).unwrap_or(true)
    }

    pub fn is_stable(&self) -> bool {
        self.clients.iter().all(|client| self.is_client_stable(client))
    }

    fn send_messages(server: &mut alisa::Server<P>, client: &TestingClient<P>) {
        let messages = client.client.take_messages();
        for message in messages {
            server.receive_message(client.id, &message);
        }
    }

    fn receive_messages(server: &mut alisa::Server<P>, client: &mut TestingClient<P>) {
        if let Some(messages) = server.get_msgs_to_send_mut(client.id) {
            let messages = std::mem::replace(messages, Vec::new());
            for message in messages {
                client.client.receive_message(&message, &mut ());
            }
        }
    }

    pub fn client(&self, id: usize) -> &alisa::Client<P> {
        &self.clients[id].client
    }

    pub fn alice(&self) -> &alisa::Client<P> {
        self.client(0)
    }

    pub fn bob(&self) -> &alisa::Client<P> {
        self.client(1)
    }

    pub fn tick_client(&mut self, id: usize) {
        self.clients[id].client.tick(&mut ());
    }

    pub fn tick_alice(&mut self) {
        self.tick_client(0);
    }

    pub fn tick_bob(&mut self) {
        self.tick_client(1);
    }

    pub fn send_alice_messages(&mut self) {
        self.tick_alice();
        Self::send_messages(&mut self.server, &self.clients[0]);
    }

    pub fn receive_alice_messages(&mut self) {
        Self::receive_messages(&mut self.server, &mut self.clients[0]);
    }

    pub fn send_bob_messages(&mut self) {
        self.tick_bob();
        Self::send_messages(&mut self.server, &self.clients[1]);
    }

    pub fn receive_bob_messages(&mut self) {
        Self::receive_messages(&mut self.server, &mut self.clients[1]);
    }

    pub fn stabilize(&mut self) {
        while !self.is_stable() {
            for client in &mut self.clients {
                client.client.tick(&mut ());
                Self::send_messages(&mut self.server, &client);
                Self::receive_messages(&mut self.server, client);
            }
        }
    }

    pub fn add_client(&mut self) -> usize {
        self.clients.push(TestingClient::new(&mut self.server));
        self.clients.len() - 1
    }

}

impl<P: alisa::Project> Drop for TestingServer<P> {

    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }

}
