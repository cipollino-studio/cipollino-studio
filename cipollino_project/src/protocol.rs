use crate::{crdt::{fractional_index::FractionalIndex, register::RegisterUpdate}, project::{folder::Folder, obj::ObjPtr}};


#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct WelcomeFolderData {
    pub parent: RegisterUpdate<(ObjPtr<Folder>, FractionalIndex)>,
    pub ptr: ObjPtr<Folder>,
    pub name: RegisterUpdate<String>,
    pub children: Vec<WelcomeFolderData>
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
        name: RegisterUpdate<String>,
        parent: RegisterUpdate<(ObjPtr<Folder>, FractionalIndex)> 
    },
    SetFolderName {
        ptr: ObjPtr<Folder>,
        name_update: RegisterUpdate<String> 
    },
    TransferFolder {
        ptr: ObjPtr<Folder>,
        parent_update: RegisterUpdate<(ObjPtr<Folder>, FractionalIndex)>
    }
}
