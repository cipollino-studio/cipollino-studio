
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use project::{alisa, ClientId, Message, PresenceData, WelcomeMessage, PROTOCOL_VERSION};
use alisa::Serializable;
use warp::ws;
use futures::SinkExt;
use tokio::sync::Mutex;

pub struct Server {
    server: project::Server,
    clients: HashMap<ClientId, Client>
}

pub struct Client {
    sender: futures::stream::SplitSink<ws::WebSocket, ws::Message>,
    presence: PresenceData 
}

impl Client {

    async fn send<T: alisa::Serializable>(&mut self, msg: T) -> bool {
        let data = msg.shallow_serialize();
        let data = alisa::encode_abf(&data);
        self.sender.send(ws::Message::binary(data)).await.is_ok()
    }

}

impl Server {

    pub fn new(path: PathBuf) -> Self {
        Self {
            server: project::Server::new(path, ()).expect("could not open project"),
            clients: HashMap::new()
        }
    }

    async fn process_message(&mut self, client_id: ClientId, msg: &Message) {
        match msg {
            Message::Collab(msg) => {
                self.server.receive_message(client_id, msg);
            },
            Message::Presence(presence_data) => {
                for (other_client_id, other_client) in &mut self.clients {
                    if *other_client_id == client_id {
                        continue;
                    }
                    other_client.send(Message::PresenceUpdate(client_id, presence_data.clone())).await;
                }
                if let Some(client) = self.clients.get_mut(&client_id) {
                    client.presence = presence_data.clone();
                }
            },
            _ => {}
        }
    }

    async fn receive_message(&mut self, client_id: ClientId, msg: alisa::ABFValue) {
        if let Some(msgs) = msg.as_array() {
            for submsg in msgs {
                let Some(submsg) = Message::data_deserialize(submsg) else { continue; };
                self.process_message(client_id, &submsg).await;
            }
        } else {
            let Some(msg) = Message::data_deserialize(&msg) else { return; };
            self.process_message(client_id, &msg).await;
        }
        for (client_id, msgs) in self.server.take_all_msgs_to_send() {
            if let Some(client) = self.clients.get_mut(&client_id) {
                if !msgs.is_empty() {
                    client.send(
                        alisa::ABFValue::Array(
                            msgs.into_iter()
                                .map(|msg| Message::Collab(msg).shallow_serialize())
                                .collect()
                        )
                    ).await;
                }
            }
        }
    }

    pub async fn handle_connection(server_arc: Arc<Mutex<Self>>, socket: ws::WebSocket) {
        use futures::StreamExt;
        println!("New client connected.");

        let mut server = server_arc.lock().await;
        let (sender, mut receiver) = socket.split();
        let (client_id, welcome_msg) = server.server.add_client();
        let welcome_msg = WelcomeMessage {
            collab: welcome_msg,
            version: PROTOCOL_VERSION,
            presence: server.clients.iter().map(|(id, client)| (*id, client.presence.clone())).collect(),
        };

        let mut client = Client {
            sender,
            presence: Default::default() 
        };

        client.send(welcome_msg).await;
        server.clients.insert(client_id, client);
        drop(server);

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
            client.send(Message::Disconnect(client_id)).await; 
        }
    }

}
