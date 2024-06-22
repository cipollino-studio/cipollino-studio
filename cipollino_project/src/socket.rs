
pub use ewebsock::{WsEvent, WsMessage};

use crate::protocol::Message;

pub struct Socket {
    sender: ewebsock::WsSender,
    receiver: ewebsock::WsReceiver,
}

impl Socket {

    pub fn new<F>(url: &str, wakeup: F) -> Result<Self, String> where F: Fn() + Sync + Send + 'static {
        let (sender, receiver) = ewebsock::connect_with_wakeup(url, Default::default(), wakeup)?;
        Ok(Self {
            sender,
            receiver
        })
    }

    pub fn receive(&self) -> Option<WsEvent> {
        self.receiver.try_recv()
    }

    pub fn send(&mut self, message: Message) {
        if let Ok(data) = bson::to_vec(&message) {
            self.sender.send(WsMessage::Binary(data));
        }
    }

}