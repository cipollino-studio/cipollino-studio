
use crate::{ClientId, Clip};

#[derive(Clone, alisa::Serializable)]
pub struct PresenceData {
    pub open_clip: alisa::Ptr<Clip>,
    pub mouse_pos: Option<[f32; 2]>
}

impl Default for PresenceData {

    fn default() -> Self {
        Self {
            open_clip: alisa::Ptr::null(),
            mouse_pos: None 
        }
    }

}

#[derive(alisa::Serializable)]
pub enum Message {
    Collab(alisa::Message),
    Presence(PresenceData),
    PresenceUpdate(ClientId, PresenceData),
    Disconnect(ClientId)
}
