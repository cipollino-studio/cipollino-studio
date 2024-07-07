

use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::{crdt::register::Register, project::{obj::{ChildList, ObjPtr}, Project}, protocol::{Message, ObjMessage, WelcomeData, WelcomeFolderData}, serialization::Serializer};

use futures::{channel::mpsc::UnboundedSender, future, pin_mut, StreamExt};
use tokio::net::{TcpListener, TcpStream};

struct Client {
    tx: UnboundedSender<Message>
}

impl Client {

    pub fn send(&mut self, msg: Message) {
        let _ = self.tx.unbounded_send(msg); 
    }

}

pub struct ProjectServer { 
    pub project: Project,
    curr_key: u64,
    serializer: Serializer,

    clients: HashMap<u64, Client>,
    curr_client_id: u64
}

include!("server.gen.rs");

impl ProjectServer {

    async fn handle_connection(stream: TcpStream, server: Arc<Mutex<Self>>) -> Option<()> {
        let ws_stream = tokio_tungstenite::accept_async(stream).await.ok()?;

        let (send, recv) = ws_stream.split();
        let (tx, rx) = futures::channel::mpsc::unbounded(); 

        let id = server.lock().unwrap().add_client(tx);
        println!("New client connected (ID: {})", id);

        let handle_recv = recv.for_each(|msg| {
            let msg = match msg {
                Ok(msg) => msg,
                Err(_) => return future::ready(()),
            };
            let msg = match msg {
                tokio_tungstenite::tungstenite::Message::Binary(msg) => msg,
                _ => return future::ready(())
            };
            let msg = match bson::from_slice::<Message>(&msg) {
                Ok(msg) => msg,
                Err(_) => return future::ready(()),
            };

            server.lock().unwrap().handle_message(id, msg);

            future::ready(())
        });

        let handle_tx = rx.map(|msg| {
            let encoded_msg = bson::to_vec(&msg).unwrap();
            Ok(tokio_tungstenite::tungstenite::Message::binary(encoded_msg))
        }).forward(send); 

        pin_mut!(handle_recv, handle_tx);
        future::select(handle_recv, handle_tx).await;

        println!("Client disconnected (ID: {})", id);
        server.lock().unwrap().remove_client(id);

        Some(())

    }

    pub async fn start(addr: String, project: Project, curr_key: u64, serializer: Serializer) {
        let listener = TcpListener::bind(&addr).await.unwrap();
        let server = Arc::new(Mutex::new(Self::new(project, curr_key, serializer)));
        while let Ok((stream, _addr)) = listener.accept().await {
            tokio::spawn(Self::handle_connection(stream, server.clone()));
        }
    }

    fn new(project: Project, curr_key: u64, serializer: Serializer) -> Self {
        Self {
            project,
            curr_key,
            serializer,
            curr_client_id: 1,
            clients: HashMap::new()
        } 
    }

    pub fn add_client(&mut self, tx: UnboundedSender<Message>) -> u64 {
        let id = self.curr_client_id;
        self.curr_client_id += 1;

        let mut client = Client {
            tx
        };

        client.send(self.get_welcome_message(id));

        self.clients.insert(id, client);
        id
    }

    pub fn remove_client(&mut self, id: u64) {
        self.clients.remove(&id);
    }

    pub fn broadcast(&mut self, msg: Message, except: Option<u64>) {
        for (id, client) in self.clients.iter_mut() {
            if Some(*id) != except {
                client.send(msg.clone());
            }
        }
    }

    pub fn send(&mut self, msg: Message, client: u64) {
        if let Some(client) = self.clients.get_mut(&client) {
            client.send(msg);
        }
    }

    fn get_folder_data(&self, folder_ptr: ObjPtr<Folder>) -> WelcomeFolderData {
        let folder = self.project.folders.get(folder_ptr).unwrap();
        WelcomeFolderData {
            ptr: folder_ptr, 
            parent: folder.folder.to_update(),
            children: folder.folders.objs.iter().map(|(_idx, ptr)| self.get_folder_data(*ptr)).collect(),
            clips: folder.clips.iter().map(|ptr| self.get_clip_data(ptr)).collect(),
            name: folder.name.to_update(),
        }
    }

    fn get_welcome_message(&mut self, client_id: u64) -> Message {
        Message::Welcome(WelcomeData {
            client_id,
            fps: self.project.fps,
            sample_rate: self.project.sample_rate,
            root_folder_data: self.get_folder_data(self.project.root_folder)
        })
    }
    
    pub fn handle_message(&mut self, client_id: u64, msg: Message) -> Option<()> {

        match msg {
            Message::KeyRequest { amount } => {
                let (first, last) = self.alloc_key_range(amount);

                if let Some(client) = self.clients.get_mut(&client_id) {
                    client.send(Message::KeyGrant {
                        first,
                        last
                    });
                }
            },
            Message::Obj(msg) => {
                if let ObjMessage::TransferFolder { ptr, parent_update } = &msg {
                    let ptr = *ptr;
                    let parent_update = parent_update.clone();

                    // Ensure the new folder exists
                    self.project.folders.get(parent_update.value.0)?;

                    if Folder::is_inside(&self.project, ptr, parent_update.value.0) {
                        // A cycle was detected - revert their transfer
                        let folder = self.project.folders.get_mut(ptr)?;
                        let update = folder.folder.set(folder.folder.value.clone())?;
                        // Broadcast just to be safe - ensure everyone has the same time/client_id for the parent register
                        self.broadcast(Message::Obj(ObjMessage::TransferFolder {
                            ptr,
                            parent_update: update
                        }), None); 
                        return Some(());
                    }

                    let folder = self.project.folders.get_mut(ptr)?;
                    let old_parent = folder.folder.0;
                    if folder.folder.apply(parent_update.clone()) {
                        self.project.folders.get_mut(old_parent)?.folders.remove(ptr); 
                        self.project.folders.get_mut(parent_update.value.0)?.folders.insert(parent_update.value.1.clone(), ptr);
                    }

                    self.serializer.set_obj_data(&self.project, ptr);
                    self.serializer.set_obj_data(&self.project, old_parent);
                    self.serializer.set_obj_data(&self.project, parent_update.value.0);

                    self.broadcast(Message::Obj(ObjMessage::TransferFolder {
                        ptr,
                        parent_update,
                    }), Some(client_id));

                } else {
                    self.handle_obj_message(client_id, msg);
                }
            },
            Message::LoadRequest(request) => { self.handle_load_request(client_id, request); },
            // These messages only need to be handled by the client
            Message::LoadResult(_) => {}, 
            Message::Welcome(_) => {},
            Message::KeyGrant { .. } => {},
        }

        Some(())
    }

    pub fn alloc_key_range(&mut self, cnt: u64) -> (u64, u64) {
        let first = self.curr_key;
        self.curr_key += cnt;
        let last = self.curr_key - 1;
        (first, last)
    }

}
