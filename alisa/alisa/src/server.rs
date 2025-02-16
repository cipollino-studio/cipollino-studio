
use std::{collections::HashMap, fmt::Debug, path::Path};

use crate::{Client, Serializable, Project, SerializationContext};

struct ServerClient {
    to_send: Vec<rmpv::Value>
}

pub struct Server<P: Project> {
    /// The pseudo-client holding the server's project. Handles receiving operation messages and serialization.
    client: Client<P>,
    context: P::Context,
    curr_client_id: u64,
    clients: HashMap<ClientId, ServerClient>
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct ClientId(u64);

impl Debug for ClientId {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }

}

impl<P: Project> Serializable<P> for ClientId {

    fn deserialize(data: &rmpv::Value, _context: &mut crate::DeserializationContext<P>) -> Option<Self> {
        data.as_u64().map(|id| Self(id))
    }
    
    fn serialize(&self, _context: &SerializationContext<P>) -> rmpv::Value {
        self.0.into()
    }

}

impl<P: Project> Server<P> {

    pub fn new<PathRef: AsRef<Path>>(path: PathRef, context: P::Context) -> Option<Self> {
        let client = Client::local(path)?;
        Some(Self {
            client,
            context,
            curr_client_id: 1,
            clients: HashMap::new()
        })
    }

    pub fn add_client(&mut self) -> (ClientId, rmpv::Value) {
        let id = ClientId(self.curr_client_id);
        self.curr_client_id += 1;

        self.clients.insert(id, ServerClient {
            to_send: Vec::new()
        });

        let storing_context = SerializationContext::deep(&self.client.objects);
        let project_data = self.client.project.serialize(&storing_context); 

        (id, rmpv::Value::Map(vec![
            ("id".into(), id.0.into()),
            ("project".into(), project_data)
        ]))
    }

    fn send(&mut self, to: ClientId, msg: rmpv::Value) -> Option<()> {
        self.clients.get_mut(&to)?.to_send.push(msg);
        Some(())
    }

    fn broadcast(&mut self, msg: &rmpv::Value, except: Option<ClientId>) {
        for (client_id, client) in self.clients.iter_mut() {
            if Some(*client_id) != except {
                client.to_send.push(msg.clone());
            }
        }
    }

    pub fn receive_message(&mut self, client_id: ClientId, msg: rmpv::Value) -> Option<()> {
        let msg = msg.as_map()?;

        let mut msg_type = "";
        let mut operation_name = "";
        let mut data = None;
        let mut object = "";
        let mut load_key = 0;

        for (key, val) in msg {
            match key.as_str()? {
                "type" => {
                    msg_type = val.as_str()?;
                },
                "operation" => {
                    operation_name = val.as_str()?;
                },
                "data" => {
                    data = Some(val);
                },
                "key" => {
                    load_key = val.as_u64()?;
                },
                "object" => {
                    object = val.as_str()?;
                },
                _ => {}
            } 
        }

        match msg_type {
            "operation" => {
                if let Some(data) = data {
                    if self.client.handle_operation_message(operation_name, data, &mut self.context).is_some() {
                        self.broadcast(&rmpv::Value::Map(vec![
                            ("type".into(), "operation".into()),
                            ("operation".into(), operation_name.into()),
                            ("data".into(), data.clone())
                        ]), Some(client_id));
                    }
                    self.send(client_id, rmpv::Value::Map(vec![
                        ("type".into(), "confirm".into())
                    ]));
                }
            },
            "key_request" => {
                // TODO: make sure the client isn't requesting too many keys
                let (first, last) = self.client.kind.as_local().expect("server should only use local client.").next_key_range(512);
                self.send(client_id, rmpv::Value::Map(vec![
                    ("type".into(), "key_grant".into()),
                    ("first".into(), first.into()),
                    ("last".into(), last.into())
                ]));
            },
            "load" => {
                for object_kind in P::OBJECTS {
                    if object_kind.name == object {
                        let local = self.client.kind.as_local().unwrap();
                        local.dyn_load(&object_kind, &mut self.client.objects, load_key);
                        let data = (object_kind.serialize_object)(&mut self.client.objects, load_key);
                        if let Some(data) = data {
                            self.send(client_id, rmpv::Value::Map(vec![
                                ("type".into(), "load".into()),
                                ("object".into(), object.into()),
                                ("key".into(), load_key.into()),
                                ("data".into(), data)
                            ]));
                        }
                        break;
                    }
                }
            },
            _ => {}
        }

        self.client.tick(&mut self.context);

        Some(())
    }

    pub fn project(&self) -> &P {
        &self.client.project
    }

    pub fn get_msgs_to_send(&mut self, client: ClientId) -> Option<&mut Vec<rmpv::Value>> {
        Some(&mut self.clients.get_mut(&client)?.to_send)
    }

    pub fn take_all_msgs_to_send(&mut self) -> HashMap<ClientId, Vec<rmpv::Value>> {
        self.clients.iter_mut().map(|(id, client)| (*id, std::mem::replace(&mut client.to_send, Vec::new()))).collect()
    }

}
