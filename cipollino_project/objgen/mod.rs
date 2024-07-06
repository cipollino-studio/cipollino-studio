
mod obj;
mod protocol;
mod client_collab;
mod client;
mod action;
mod server;

use action::generate_action_code;
use bitflags::bitflags;
use client::generate_client_code;
use client_collab::generate_client_collab_code;
use obj::generate_obj_code;
use protocol::generate_protocol_code;
use server::generate_server_code;

pub struct ObjField {
    name: &'static str,
    ty: &'static str,
    default: &'static str
}

bitflags! {

    #[derive(Clone, Copy)]
    struct ObjTypeFlags: u64 {
        const NoTransferGen = 1 << 0;
        const Asset = 1 << 1;
    }

}

pub struct ObjType {
    name: &'static str,
    list_name: &'static str,
    parent: &'static str,
    children: &'static [&'static str],
    fields: &'static [ObjField],
    flags: ObjTypeFlags
}

pub static OBJ_TYPES: [ObjType; 2] = [
    ObjType {
        name: "Folder",
        list_name: "folders",
        parent: "Folder",
        children: &["Folder", "Clip"],
        fields: &[
            ObjField {
                name: "name",
                ty: "String",
                default: "\"Folder\".to_owned()"
            }
        ],
        flags: ObjTypeFlags::NoTransferGen
    },
    ObjType {
        name: "Clip",
        list_name: "clips",
        parent: "Folder",
        children: &[],
        fields: &[
            ObjField {
                name: "name",
                ty: "String",
                default: "\"Clip\".to_owned()"
            }
        ],
        flags: ObjTypeFlags::Asset
    }
];

impl ObjType {

    pub fn is_asset(&self) -> bool {
        self.flags.intersects(ObjTypeFlags::Asset)
    }

}

pub fn find_obj_type(name: &'static str) -> &'static ObjType {
    for obj_type in &OBJ_TYPES {
        if obj_type.name == name {
            return obj_type;
        }
    }
    panic!("obj type {} not found", name);
}

pub fn generate() {
    generate_obj_code();
    generate_protocol_code();
    generate_client_collab_code();
    generate_client_code();
    generate_action_code();
    generate_server_code();
}
