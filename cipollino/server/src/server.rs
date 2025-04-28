
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use project::{alisa::{self, ABFValue}, ClientId, Message, PresenceData, WelcomeMessage, PROTOCOL_VERSION};
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

    async fn send(&mut self, msg: ABFValue) -> bool {
        let data = alisa::encode_abf(&msg);
        self.sender.send(ws::Message::binary(data)).await.is_ok()
    }

}

impl Server {

    pub fn new(path: PathBuf) -> Self {
        Self {
            server: project::Server::new(path).expect("could not open project"),
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
                    other_client.send(self.server.serialize(*other_client_id, &Message::PresenceUpdate(client_id, presence_data.clone()))).await;
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
                let Some(submsg) = self.server.deserialize::<Message>(client_id, submsg.clone()) else { continue; };
                self.process_message(client_id, &submsg).await;
            }
        } else {
            let Some(msg) = self.server.deserialize::<Message>(client_id, msg) else { return; };
            self.process_message(client_id, &msg).await;
        }

        // Send outgoing messages
        for (client_id, msgs) in self.server.take_all_msgs_to_send() {
            if let Some(client) = self.clients.get_mut(&client_id) {
                if !msgs.is_empty() {
                    client.send(
                        alisa::ABFValue::Array(
                            msgs.into_iter()
                                .map(|msg| self.server.serialize(client_id, &Message::Collab(msg)))
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

        client.send(server.server.serialize(client_id, &welcome_msg)).await;
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
        let server = &mut *server;
        server.clients.remove(&client_id);

        let clients = &mut server.clients;
        let server = &mut server.server;

        // Tell the other clients this client disconnected
        for (other_client, client) in clients {
            client.send(server.serialize(*other_client, &Message::Disconnect(client_id))).await; 
        }
    }

}
