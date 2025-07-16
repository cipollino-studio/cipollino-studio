
use std::{collections::{HashMap, HashSet}, fmt::Debug, path::Path};

use crate::{deserialize, serialize, ABFValue, AnyPtr, Client, DeserializationContext, Message, ObjectKind, Project, Serializable, SerializationContext, WelcomeMessage, WelcomeObject};

struct ServerClient {
    to_send: Vec<Message>,
    to_server_key: HashMap<u64, u64>,
    to_client_key: HashMap<u64, u64>
}

pub struct Server<P: Project> {
    /// The pseudo-client holding the server's project. Handles receiving operation messages and serialization.
    client: Client<P>,
    curr_client_id: u64,
    clients: HashMap<ClientId, ServerClient>
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct ClientId(pub u64);

impl Debug for ClientId {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }

}

impl Serializable for ClientId {

    fn deserialize(data: &ABFValue, _context: &mut DeserializationContext) -> Option<Self> {
        data.as_u64().map(|id| Self(id))
    }
    
    fn serialize(&self, _context: &SerializationContext) -> ABFValue {
        self.0.into()
    }

    fn delete(&self, _: &mut Vec<AnyPtr>) {
        
    }

}

impl<P: Project> Server<P> {

    pub fn new<PathRef: AsRef<Path>>(path: PathRef) -> Option<Self> {
        let client = Client::local(path)?;
        Some(Self {
            client,
            curr_client_id: 1,
            clients: HashMap::new()
        })
    }

    pub fn add_client(&mut self) -> (ClientId, WelcomeMessage) {
        let id = ClientId(self.curr_client_id);
        self.curr_client_id += 1;

        self.clients.insert(id, ServerClient {
            to_send: Vec::new(),
            to_server_key: HashMap::new(),
            to_client_key: HashMap::new()
        });

        let storing_context = SerializationContext::new();
        let project_data = self.client.project.serialize(&storing_context); 

        let mut welcome_objects = Vec::new(); 
        let mut encoded = HashSet::new();
        let mut to_encode = storing_context.take_serialization_requests();
        while let Some((obj_type, key)) = to_encode.pop() {
            encoded.insert(key);
            let context = SerializationContext::new();
            let Some(object_data) = (P::OBJECTS[obj_type as usize].serialize_object)(&mut self.client.objects, key, &context) else { continue; };
            for (next_obj_type, next_key) in context.take_serialization_requests() {
                if !encoded.contains(&next_key) {
                    to_encode.push((next_obj_type, next_key));
                }
            }
            welcome_objects.push(WelcomeObject {
                ptr: AnyPtr::new(obj_type, key),
                obj: object_data,
            });
        }

        (id, WelcomeMessage {
            id,
            project: project_data,
            objects: welcome_objects,
        })
    }

    pub fn send(&mut self, to: ClientId, msg: Message) -> Option<()> {
        self.clients.get_mut(&to)?.to_send.push(msg);
        Some(())
    }

    pub fn broadcast(&mut self, msg: &Message, except: Option<ClientId>) {
        for (client_id, client) in self.clients.iter_mut() {
            if Some(*client_id) != except {
                client.to_send.push(msg.clone());
            }
        }
    }

    pub fn receive_message(&mut self, client_id: ClientId, msg: &Message) {
        match msg {
            Message::Operation { operation, data } => {
                if self.client.handle_operation_message(operation, data) {
                    self.broadcast(msg, Some(client_id));
                }
                self.send(client_id, Message::ConfirmOperation);
            },
            Message::LoadRequest { ptr } => {
                let obj_type = ptr.obj_type();
                let key = ptr.key();
                let object_kind = &P::OBJECTS[obj_type as usize];
                self.handle_load_message(object_kind, key, client_id); 
            },
            _ => {
                return;
            }
        }
        self.client.tick();
    }

    fn handle_load_message(&mut self, object_kind: &ObjectKind<P>, key: u64, client_id: ClientId) {
        let mut to_encode = vec![(object_kind, key)];
        let mut encoded = HashSet::new();
        while let Some((object_kind, key)) = to_encode.pop() {
            encoded.insert(key);

            let local = self.client.kind.as_local().unwrap();
            local.dyn_load(&object_kind, &mut self.client.objects, key);
            let context = SerializationContext::new();
            let data = (object_kind.serialize_object)(&mut self.client.objects, key, &context);

            let ptr = AnyPtr::new(object_kind.object_type_id, key);
            if let Some(data) = data {
                self.send(client_id, Message::Load {
                    ptr,
                    obj: data
                });
            } else {
                self.send(client_id, Message::LoadFailed { ptr });
            }

            for (obj_type, key) in context.take_serialization_requests() {
                if !encoded.contains(&key) {
                    to_encode.push((&P::OBJECTS[obj_type as usize], key));
                }
            }
        }
        
    }

    pub fn project(&self) -> &P {
        &self.client.project
    }

    pub fn get_msgs_to_send(&self, client: ClientId) -> Option<&Vec<Message>> {
        Some(&self.clients.get(&client)?.to_send)
    }

    pub fn get_msgs_to_send_mut(&mut self, client: ClientId) -> Option<&mut Vec<Message>> {
        Some(&mut self.clients.get_mut(&client)?.to_send)
    }

    pub fn take_all_msgs_to_send(&mut self) -> HashMap<ClientId, Vec<Message>> {
        self.clients.iter_mut().map(|(id, client)| (*id, std::mem::replace(&mut client.to_send, Vec::new()))).collect()
    }

    fn map_to_client_keys(data: &mut ABFValue, to_client_key: &HashMap<u64, u64>) {
        match data {
            ABFValue::ObjPtr(_, key) => {
                if let Some(mapped_key) = to_client_key.get(&key) {
                    *key = *mapped_key;
                }
            },
            ABFValue::Array(items) => {
                for item in items {
                    Self::map_to_client_keys(item, to_client_key);
                }
            },
            ABFValue::Map(items) => {
                for (_, item) in items {
                    Self::map_to_client_keys(item, to_client_key);
                }
            },
            ABFValue::IndexedEnum(_, data) => {
                Self::map_to_client_keys(data, to_client_key);
            },
            ABFValue::NamedEnum(_, data) => {
                Self::map_to_client_keys(data, to_client_key);
            },
            _ => {}
        }
    }

    /// Serialize an object, taking key mapping into account
    pub fn serialize<T: Serializable>(&self, client: ClientId, obj: &T) -> ABFValue {
        let mut data = serialize(obj);
        if let Some(client) = self.clients.get(&client) {
            Self::map_to_client_keys(&mut data, &client.to_client_key);
        }
        data
    }

    fn map_to_server_keys(data: &mut ABFValue, to_client_key: &mut HashMap<u64, u64>, to_server_key: &mut HashMap<u64, u64>, client: &Client<P>) {
        match data {
            ABFValue::ObjPtr(_, key) => {
                if let Some(mapped_key) = to_server_key.get(&key) {
                    *key = *mapped_key;
                } else if *key & (1 << 63) > 0 {
                    let server_key = client.next_key();
                    to_server_key.insert(*key, server_key);
                    to_client_key.insert(server_key, *key);
                    *key = server_key;
                }
            },
            ABFValue::Array(items) => {
                for item in items {
                    Self::map_to_server_keys(item, to_client_key, to_server_key, client);
                }
            },
            ABFValue::Map(items) => {
                for (_, item) in items {
                    Self::map_to_server_keys(item, to_client_key, to_server_key, client);
                }
            },
            ABFValue::IndexedEnum(_, data) => {
                Self::map_to_server_keys(data, to_client_key, to_server_key, client);
            },
            ABFValue::NamedEnum(_, data) => {
                Self::map_to_server_keys(data, to_client_key, to_server_key, client);
            },
            _ => {}
        }
    }

    /// Deserialize an object, taking key mapping into account
    pub fn deserialize<T: Serializable>(&mut self, client: ClientId, mut data: ABFValue) -> Option<T> {
        if let Some(client) = self.clients.get_mut(&client) {
            Self::map_to_server_keys(&mut data, &mut client.to_client_key, &mut client.to_server_key, &self.client);
        }
        deserialize::<T>(&data)
    }

}
