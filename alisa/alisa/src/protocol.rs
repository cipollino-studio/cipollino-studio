
use crate::{alisa, ABFValue, AnyPtr, ClientId, Serializable};

#[derive(Serializable)]
pub struct WelcomeObject {
    pub ptr: AnyPtr,
    pub obj: ABFValue
}

impl Default for WelcomeObject {

    fn default() -> Self {
        Self {
            ptr: Default::default(),
            obj: ABFValue::PositiveInt(0) 
        }
    }

}

#[derive(Serializable)]
pub struct WelcomeMessage {
    pub id: ClientId,
    pub project: ABFValue,
    pub objects: Vec<WelcomeObject>
}

impl Default for WelcomeMessage {

    fn default() -> Self {
        Self {
            id: Default::default(),
            project: ABFValue::PositiveInt(0),
            objects: Vec::new()
        }
    }
    
}

#[derive(Clone, Serializable)]
pub enum Message {
    Operation {
        operation: String,
        data: ABFValue
    },
    ConfirmOperation,
    KeyRequest,
    KeyGrant {
        first: u64,
        last: u64
    },
    LoadRequest {
        ptr: AnyPtr
    },
    Load {
        ptr: AnyPtr,
        obj: ABFValue
    },
    LoadFailed {
        ptr: AnyPtr
    }
}
