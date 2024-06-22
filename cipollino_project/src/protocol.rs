use crate::{crdt::{fractional_index::FractionalIndex, register::{Register, RegisterUpdate}}, project::{folder::Folder, obj::ObjPtr}};


#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct WelcomeFolderData {
    pub ptr: ObjPtr<Folder>,
    pub name: Register<String>,
    pub children: Vec<(FractionalIndex, WelcomeFolderData)>
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
    AddFolder {
        ptr: ObjPtr<Folder>,
        idx: FractionalIndex,
        name: RegisterUpdate<String>,
        parent: ObjPtr<Folder> 
    },
    SetFolderName {
        ptr: ObjPtr<Folder>,
        name_update: RegisterUpdate<String> 
    }
}
