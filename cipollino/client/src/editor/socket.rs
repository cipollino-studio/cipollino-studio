use project::alisa::{rmpv, rmpv_decode, rmpv_encode};

#[derive(PartialEq, Eq)]
enum SocketState {
    None,
    Opened,
    Closed
}

pub struct Socket {
    sender: ewebsock::WsSender,
    receiver: ewebsock::WsReceiver,
    state: SocketState,
    error: Option<String> 
}

impl Socket {

    pub fn new(url: &str) -> Result<Self, String> {
        let (sender, receiver) = ewebsock::connect(url, ewebsock::Options::default())?;        
        Ok(Self {
            sender,
            receiver,
            state: SocketState::None,
            error: None
        })
    }

    pub fn receive(&mut self) -> Option<rmpv::Value> {
        let event = self.receiver.try_recv()?;
        match event {
            ewebsock::WsEvent::Opened => {
                self.state = SocketState::Opened;
                None
            },
            ewebsock::WsEvent::Message(msg) => {
                if let ewebsock::WsMessage::Binary(data) = msg {
                    let msg = rmpv_decode(&data)?;
                    Some(msg)
                } else {
                    None
                }
            },
            ewebsock::WsEvent::Error(msg) => {
                self.error = Some(msg);
                self.state = SocketState::Closed;
                None
            },
            ewebsock::WsEvent::Closed => {
                self.state = SocketState::Closed;
                None
            },
        }
    }

    pub fn send(&mut self, msg: rmpv::Value) {
        let Some(data) = rmpv_encode(&msg) else { return; };
        let msg = ewebsock::WsMessage::Binary(data);
        self.sender.send(msg);
    }

    pub fn opened(&self) -> bool {
        self.state == SocketState::Opened
    }

    pub fn closed(&self) -> bool {
        self.state == SocketState::Closed
    }

    pub fn take_error(&mut self) -> Option<String> {
        self.error.take()
    }

}
