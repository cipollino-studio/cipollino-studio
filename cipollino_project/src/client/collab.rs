use ewebsock::{WsEvent, WsMessage};

use crate::{crdt::register::Register, project::{folder::Folder, obj::{ChildList, ObjPtr}, Project}, protocol::{Message, WelcomeData, WelcomeFolderData}, socket::Socket};

use super::ProjectClient;

pub(crate) struct KeyBlock {
    curr: u64,
    last: u64,
    key_request_sent: bool
}

impl KeyBlock {
    
    pub fn empty() -> Self {
        Self {
            curr: 1,
            last: 0,
            key_request_sent: false
        }
    }

    pub fn is_empty(&self) -> bool {
        self.curr > self.last
    }

    pub fn next(&mut self) -> Option<u64> {
        if self.is_empty() {
            None
        } else {
            self.curr += 1;
            Some(self.curr - 1)
        }
    }
    
}

const N_KEY_BLOCKS: usize = 2;

pub struct Collab {
    pub(crate) socket: Socket,
    pub(crate) keys: [KeyBlock; N_KEY_BLOCKS],
    pub(crate) client_id: u64
}

impl Collab {

    pub(super) fn has_keys(&self) -> bool {
        for block in &self.keys {
            if !block.is_empty() {
                return true
            }
        }
        false
    }

    pub(super) fn next_key(&mut self) -> Option<u64> {
        for block in &mut self.keys {
            if let Some(key) = block.next() {
                return Some(key);
            }
        }
        None
    }

    pub(super) fn update(&mut self, project: &mut Project) -> Result<(), String> {
        while let Some(event) = self.socket.receive() {
            self.handle_collab_event(event, project)?;
        }
        self.send_key_requests();
        Ok(())
    }

    fn handle_collab_event(&mut self, event: WsEvent, project: &mut Project) -> Result<(), String> {
        match event {
            WsEvent::Message(msg) => {
                let data = match msg {
                    WsMessage::Binary(data) => data,
                    _ => return Ok(())
                };
                let msg = match bson::from_slice(&data) { 
                    Ok(msg) => msg,
                    Err(_) => return Ok(()),
                };
                self.handle_collab_msg(msg, project);
            },
            WsEvent::Error(err) => {
                return Err(err);
            },
            WsEvent::Closed => {
                return Err("Collab server disconnected.".to_owned())
            },
            _ => {}
        }
        Ok(())
    }

    fn handle_collab_msg(&mut self, msg: Message, project: &mut Project) -> Option<()> {
        match msg {
            Message::KeyGrant { first, last } => self.handle_key_grant(first, last),
            Message::AddFolder { ptr, name, parent } => {
                project.add(ptr, Folder {
                    parent: Register::from_update(parent, self.client_id),
                    name: Register::from_update(name, self.client_id),
                    folders: ChildList::new(),
                });
            },
            Message::SetFolderName { ptr, name_update } => {
                let folder = project.folders.get_mut(ptr)?;
                folder.name.apply(name_update);
            },
            Message::TransferFolder {ptr, parent_update} => {
                project.folders.get(parent_update.value.0)?;

                if Folder::is_inside(project, ptr, parent_update.value.0) {
                    return None;
                }

                let folder = project.folders.get_mut(ptr)?;
                let old_parent = folder.parent.0;
                if folder.parent.apply(parent_update.clone()) {
                    project.folders.get_mut(old_parent)?.folders.remove(ptr); 
                    project.folders.get_mut(parent_update.value.0)?.folders.insert(parent_update.value.1.clone(), ptr);
                }
            },
            
            Message::Welcome(_) => {},
            Message::KeyRequest { .. } => {},
        }
        Some(())
    }
    
    fn handle_key_grant(&mut self, first: u64, last: u64) {
        for block in &mut self.keys {
            if block.key_request_sent {
                block.curr = first;
                block.last = last;
                block.key_request_sent = false;
                break;
            }
        }
    }

    fn send_key_requests(&mut self) {
        for block in &mut self.keys {
            if block.is_empty() && !block.key_request_sent {
                self.socket.send(Message::KeyRequest { amount: 1024 });
                block.key_request_sent = true;
            }
        }
    }

}

impl ProjectClient {

    fn add_folders_from_welcome_data(project: &mut Project, client_id: u64, folder_data: WelcomeFolderData) -> ObjPtr<Folder> {
        let mut children = Vec::new();
        for child in folder_data.children {
            children.push((child.parent.1.clone(), Self::add_folders_from_welcome_data(project, client_id, child)));
        }

        project.folders.objs.insert(folder_data.ptr, Folder {
            parent: Register::from_update(folder_data.parent, client_id),
            folders: ChildList {
                objs: children 
            },
            name: Register::from_update(folder_data.name, client_id),
        });
        folder_data.ptr 
    }

    pub fn collab(socket: Socket, welcome_data: WelcomeData) -> (ProjectClient, Project) {
        let mut project = Project::empty(welcome_data.fps, welcome_data.sample_rate);

        project.root_folder = Self::add_folders_from_welcome_data(&mut project, welcome_data.client_id, welcome_data.root_folder_data);

        let client = ProjectClient::Collab(Collab {
            socket,
            keys: [KeyBlock::empty(), KeyBlock::empty()],
            client_id: welcome_data.client_id,
        });

        (client, project)
    }

}
