
use std::sync::{Arc, Mutex};

use project::Message;
use project::alisa::Serializable;

#[derive(PartialEq, Eq)]
enum SocketState {
    None,
    Opened,
    Closed
}

pub struct Socket {
    sender: ewebsock::WsSender,
    state: Arc<Mutex<SocketState>>,
    error: Arc<Mutex<Option<String>>>,
    msgs: Arc<Mutex<Vec<alisa::ABFValue>>>,
    signal: Arc<Mutex<Option<pierro::RedrawSignal>>>,
    has_signal: bool
}

impl Socket {

    pub fn new(url: &str) -> Result<Self, String> {
        let (sender, receiver) = ewebsock::connect(url, ewebsock::Options::default())?;        

        let state = Arc::new(Mutex::new(SocketState::None));
        let error = Arc::new(Mutex::new(None));
        let msgs = Arc::new(Mutex::new(Vec::new()));
        let signal = Arc::new(Mutex::new(None::<pierro::RedrawSignal>));

        let state_copy = state.clone();
        let error_copy = error.clone();
        let msgs_copy = msgs.clone();
        let signal_copy = signal.clone();

        std::thread::spawn(move || {
            loop {
                let Some(event) = receiver.try_recv() else { continue; };
                if let Some(signal) = &*signal.lock().unwrap() {
                    signal.request_redraw();
                }
                match event {
                    ewebsock::WsEvent::Opened => {
                        *state.lock().unwrap() = SocketState::Opened;
                    },
                    ewebsock::WsEvent::Message(msg) => {
                        if let ewebsock::WsMessage::Binary(data) = msg {
                            if let Some(msg) = alisa::parse_abf(&data) {
                                msgs.lock().unwrap().push(msg);
                            }
                        }
                    },
                    ewebsock::WsEvent::Error(msg) => {
                        *error.lock().unwrap() = Some(msg);
                        *state.lock().unwrap() = SocketState::Closed;
                    },
                    ewebsock::WsEvent::Closed => {
                        *state.lock().unwrap() = SocketState::Closed;
                        break;
                    },
                }
            }
        });

        Ok(Self {
            sender,
            state: state_copy,
            error: error_copy,
            msgs: msgs_copy,
            signal: signal_copy,
            has_signal: false
        })
    }

    pub fn receive(&mut self) -> Option<alisa::ABFValue> {
        let mut msgs = self.msgs.lock().ok()?;
        if msgs.is_empty() {
            return None;
        }
        Some(msgs.remove(0))
    }

    pub fn send(&mut self, msg: Message) {
        let data = msg.shallow_serialize();
        self.send_data(data);
    }

    pub fn send_data(&mut self, data: project::alisa::ABFValue) {
        let data = alisa::encode_abf(&data);
        let msg = ewebsock::WsMessage::Binary(data);
        self.sender.send(msg);
    }

    pub fn opened(&self) -> bool {
        *self.state.lock().unwrap() == SocketState::Opened
    }

    pub fn closed(&self) -> bool {
        *self.state.lock().unwrap() == SocketState::Closed
    }

    pub fn take_error(&mut self) -> Option<String> {
        self.error.lock().unwrap().take()
    }

    pub fn has_signal(&self) -> bool {
        self.has_signal
    }

    pub fn set_signal(&mut self, signal: pierro::RedrawSignal) {
        self.has_signal = true;
        *self.signal.lock().unwrap() = Some(signal);
    }

}
