
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use project::{alisa::{rmpv, rmpv_decode, rmpv_encode}, ClientId};
use warp::ws::{Message, WebSocket};
use futures::SinkExt;
use tokio::sync::Mutex;

pub struct Server {
    server: project::Server,
    clients: HashMap<ClientId, Client>
}

pub struct Client {
    sender: futures::stream::SplitSink<WebSocket, Message>
}

impl Client {

    async fn send(&mut self, msg: rmpv::Value) -> bool {
        let Some(data) = rmpv_encode(&msg) else { return false; };
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

    async fn receive_message(&mut self, client_id: ClientId, msg: rmpv::Value) {
        self.server.receive_message(client_id, msg);
        for (client_id, msgs) in self.server.take_all_msgs_to_send() {
            if let Some(client) = self.clients.get_mut(&client_id) {
                for msg in msgs {
                    client.send(msg).await;
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
        };

        if client.send(welcome_msg).await {
            server_arc.lock().await.clients.insert(client_id, client);
        }

        while let Some(Ok(msg)) = receiver.next().await {
            let data = msg.as_bytes();
            if let Some(msg) = rmpv_decode(data) {
                server_arc.lock().await.receive_message(client_id, msg).await;
            }
        }

        println!("Client disconnected.");
        server_arc.lock().await.clients.remove(&client_id);
    }

}
