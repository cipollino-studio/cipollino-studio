
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

pub static OBJ_TYPES: [ObjType; 5] = [
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
        children: &["Layer"],
        fields: &[
            ObjField {
                name: "name",
                ty: "String",
                default: "\"Clip\".to_owned()"
            },
            ObjField {
                name: "length",
                ty: "i32",
                default: "100"
            }
        ],
        flags: ObjTypeFlags::Asset
    },
    ObjType {
        name: "Layer",
        list_name: "layers",
        parent: "Clip",
        children: &["Frame"],
        fields: &[
            ObjField {
                name: "name",
                ty: "String",
                default: "\"Layer\".to_owned()"
            },
            ObjField {
                name: "alpha",
                ty: "f32",
                default: "1.0"
            },
            ObjField {
                name: "hide",
                ty: "bool",
                default: "false"
            },
            ObjField {
                name: "lock",
                ty: "bool",
                default: "false"
            }
        ],
        flags: ObjTypeFlags::empty()
    },
    ObjType {
        name: "Frame",
        list_name: "frames",
        parent: "Layer",
        children: &["Stroke"],
        fields: &[
            ObjField {
                name: "time",
                ty: "i32",
                default: "0"
            }
        ],
        flags: ObjTypeFlags::empty()
    },
    ObjType {
        name: "Stroke",
        list_name: "strokes",
        parent: "Frame",
        children: &[],
        fields: &[
            ObjField {
                name: "pts",
                ty: "Vec<crate::project::stroke::StrokePoint>",
                default: "vec![]"
            }
        ],
        flags: ObjTypeFlags::empty()
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
