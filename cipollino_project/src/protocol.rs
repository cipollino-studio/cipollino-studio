use crate::{crdt::{fractional_index::FractionalIndex, register::RegisterUpdate}, project::obj::ObjPtr};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct WelcomeFolderData {
    pub parent: RegisterUpdate<(ObjPtr<Folder>, FractionalIndex)>,
    pub ptr: ObjPtr<Folder>,
    pub name: RegisterUpdate<String>,
    pub children: Vec<WelcomeFolderData>,
    pub clips: Vec<WelcomeClipData>
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct WelcomeData {
    pub client_id: u64,
    pub fps: f32,
    pub sample_rate: f32,
    pub root_folder_data: WelcomeFolderData
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum Message {
    Welcome(WelcomeData),
    KeyRequest {
        amount: u64
    },
    KeyGrant {
        first: u64,
        last: u64
    },
    Obj(ObjMessage),
    LoadRequest(LoadRequest),
    LoadResult(LoadResult)
}

include!("protocol.gen.rs");
