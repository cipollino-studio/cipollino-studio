
use codegen::Scope;
use convert_case::{Case, Casing};
use super::{find_obj_type, OBJ_TYPES};


// src/protocol.gen.rs
pub fn generate_protocol_code() {

    let mut scope = Scope::new();

    for obj_type in &OBJ_TYPES {
        scope.import(format!("crate::project::{}", obj_type.name.to_case(Case::Snake)).as_str(), &obj_type.name);
    }
    
    // ObjMessage
    let obj_message_enum = scope.new_enum("ObjMessage")
        .vis("pub")
        .derive("serde::Serialize")
        .derive("serde::Deserialize")
        .derive("Clone");

    for obj_type in &OBJ_TYPES {

        let parent_type = format!("RegisterUpdate<(ObjPtr<{}>, FractionalIndex)>", obj_type.parent);

        // Add message
        let add = obj_message_enum.new_variant(format!("Add{}", obj_type.name))
            .named("ptr", format!("ObjPtr<{}>", obj_type.name))
            .named("parent", &parent_type);
        for field in obj_type.fields {
            add.named(&field.name, format!("RegisterUpdate<{}>", field.ty));
        }

        // Set messages
        for field in obj_type.fields {
            obj_message_enum.new_variant(format!("Set{}{}", obj_type.name, field.name.to_case(Case::Pascal)))
                .named("ptr", format!("ObjPtr<{}>", obj_type.name))
                .named("update", format!("RegisterUpdate<{}>", field.ty));
        }

        // Transfer message
        obj_message_enum.new_variant(format!("Transfer{}", obj_type.name))
            .named("ptr", format!("ObjPtr<{}>", obj_type.name))
            .named("parent_update", parent_type);
    }

    // LoadRequest
    let load_request_enum = scope.new_enum("LoadRequest")
        .vis("pub")
        .derive("serde::Serialize")
        .derive("serde::Deserialize")
        .derive("Clone");
    for obj_type in &OBJ_TYPES {
        if !obj_type.is_asset() {
            continue;
        }
        load_request_enum.new_variant(obj_type.name)
            .tuple(format!("ObjPtr<{}>", obj_type.name).as_str());
    }

    // LoadResult
    let load_result_enum = scope.new_enum("LoadResult")
        .vis("pub")
        .derive("serde::Serialize")
        .derive("serde::Deserialize")
        .derive("Clone");
    for obj_type in &OBJ_TYPES {
        if !obj_type.is_asset() {
            continue;
        }
        let variant = load_result_enum.new_variant(obj_type.name)
            .tuple(format!("ObjPtr<{}>", obj_type.name).as_str());
        for child in obj_type.children {
            variant.tuple(format!("Vec<{}LoadData>", child).as_str());
        }
    }

    for obj_type in &OBJ_TYPES {
        let load_data = scope.new_struct(format!("{}LoadData", obj_type.name).as_str())
            .vis("pub")
            .derive("serde::Serialize")
            .derive("serde::Deserialize")
            .derive("Clone")
            .field("pub(crate) ptr", format!("ObjPtr<{}>", obj_type.name))
            .field("pub(crate) parent", format!("RegisterUpdate<(ObjPtr<{}>, FractionalIndex)>", obj_type.parent));

        for field in obj_type.fields {
            load_data.field(format!("pub(crate) {}", field.name).as_str(), format!("RegisterUpdate<{}>", field.ty));
        }
        for child in obj_type.children {
            let child_obj = find_obj_type(child);
            load_data.field(format!("pub(crate) {}", child_obj.list_name).as_str(), format!("Vec<{}LoadData>", child_obj.name));
        }
    }

    // Welcome Data
    for obj_type in &OBJ_TYPES {
        if !obj_type.is_asset() {
            continue;
        } 

        let welcome_data_struct = scope.new_struct(format!("Welcome{}Data", obj_type.name).as_str())
            .field("pub parent", "RegisterUpdate<(ObjPtr<Folder>, FractionalIndex)>")
            .field("pub ptr", format!("ObjPtr<{}>", obj_type.name))
            .derive("serde::Serialize")
            .derive("serde::Deserialize")
            .derive("Clone")
            .vis("pub");

        for field in obj_type.fields {
            welcome_data_struct.field(format!("pub {}", field.name).as_str(), format!("RegisterUpdate<{}>", field.ty));
        }
    }

    let _ = std::fs::write("src/protocol.gen.rs", scope.to_string());

}