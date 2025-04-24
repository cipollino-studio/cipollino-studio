
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use project::{alisa, ClientId};
use warp::ws::{Message, WebSocket};
use futures::SinkExt;
use tokio::sync::Mutex;

pub struct Server {
    server: project::Server,
    clients: HashMap<ClientId, Client>
}

pub struct Client {
    sender: futures::stream::SplitSink<WebSocket, Message>,
    presence: Option<alisa::ABFValue>
}

impl Client {

    async fn send(&mut self, msg: alisa::ABFValue) -> bool {
        let data = alisa::encode_abf(&msg);
        self.sender.send(Message::binary(data)).await.is_ok()
    }

}

impl Server {

    pub fn new(path: PathBuf) -> Self {
        Self {
            server: project::Server::new(path, ()).expect("could not open project"),
            clients: HashMap::new()
        }
    }

    fn process_message(&mut self, client_id: ClientId, msg: &alisa::ABFValue) {
        let (msg_type, data) = match msg {
            alisa::ABFValue::NamedUnitEnum(name) => (name.as_str(), &alisa::ABFValue::PositiveInt(0)),
            alisa::ABFValue::NamedEnum(name, data) => (name.as_str(), &**data),
            _ => {
                return;
            }
        };
        if msg_type == "presence" {
            let Some(client) = self.clients.get_mut(&client_id) else { return; };
            self.server.broadcast(&alisa::ABFValue::NamedEnum(
                "presence".into(),
                Box::new(alisa::ABFValue::Map(Box::new([
                    ("client".into(), client_id.0.into()),
                    ("data".into(), data.clone()),
                ])))
            ), Some(client_id));
            client.presence = Some(data.clone());
            return;
        }
        self.server.receive_message(client_id, msg);
    }

    async fn receive_message(&mut self, client_id: ClientId, msg: alisa::ABFValue) {
        if let Some(msgs) = msg.as_array() {
            for submsg in msgs {
                self.process_message(client_id, submsg);
            }
        } else {
            self.process_message(client_id, &msg);
        }
        for (client_id, msgs) in self.server.take_all_msgs_to_send() {
            if let Some(client) = self.clients.get_mut(&client_id) {
                if !msgs.is_empty() {
                    client.send(alisa::ABFValue::Array(msgs.into_iter().collect())).await;
                }
            }
        }
    }

    pub async fn handle_connection(server_arc: Arc<Mutex<Self>>, socket: WebSocket) {
        use futures::StreamExt;
        println!("New client connected.");

        let (sender, mut receiver) = socket.split();
        let (client_id, welcome_msg) = server_arc.lock().await.server.add_client();

        let mut client = Client {
            sender,
            presence: None
        };

        if client.send(welcome_msg).await {
            let mut server = server_arc.lock().await;
            for (other_client_id, other_client) in &server.clients {
                let Some(presence_data) = &other_client.presence else { continue; };
                client.send(alisa::ABFValue::NamedEnum(
                    "presence".into(),
                    Box::new(alisa::ABFValue::Map(Box::new([
                        ("client".into(), other_client_id.0.into()),
                        ("data".into(), presence_data.clone()),
                    ])))
                )).await; 
            }
            server.clients.insert(client_id, client);
        }

        while let Some(Ok(msg)) = receiver.next().await {
            let data = msg.as_bytes();
            if let Some(msg) = alisa::parse_abf(data) {
                server_arc.lock().await.receive_message(client_id, msg).await;
            }
        }

        println!("Client disconnected.");
        let mut server = server_arc.lock().await;
        server.clients.remove(&client_id);

        // Tell the other clients this client disconnected
        for (_other_client, client) in &mut server.clients {
            client.send(alisa::ABFValue::NamedEnum(
                "disconnect".into(),
                Box::new(alisa::ABFValue::Map(Box::new([
                    ("type".into(), "disconnect".into()),
                    ("client".into(), client_id.0.into()),
                ])))
            )).await; 
        }
    }

}
