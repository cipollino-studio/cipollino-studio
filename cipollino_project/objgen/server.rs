
use codegen::Scope;
use convert_case::{Case, Casing};

use super::{find_obj_type, ObjTypeFlags, OBJ_TYPES};


// src/server/server.gen.rs
pub fn generate_server_code() {
    let mut scope = Scope::new();

    scope.import("crate::protocol", "LoadRequest");
    scope.import("crate::protocol", "LoadResult");
    for obj_type in &OBJ_TYPES {
        if obj_type.is_asset() {
            scope.import("crate::protocol", format!("Welcome{}Data", obj_type.name).as_str());
        }
        scope.import(format!("crate::project::{}", obj_type.name.to_case(Case::Snake)).as_str(), &obj_type.name);
        scope.import("crate::protocol", format!("{}LoadData", obj_type.name).as_str());
    }

    let project_server_impl = scope.new_impl("ProjectServer");

    // Handle message
    let handle_msg = project_server_impl.new_fn("handle_obj_message")
        .arg_mut_self()
        .arg("client_id", "u64")
        .arg("msg", "ObjMessage")
        .ret("Option<()>");

    handle_msg.line("match msg {");
    for obj_type in &OBJ_TYPES {

        // Add
        handle_msg.line(format!("\tObjMessage::Add{} {{", obj_type.name));
        handle_msg.line("\t\tptr,");
        handle_msg.line("\t\tparent,");
        for field in obj_type.fields {
            handle_msg.line(format!("\t\t{},", field.name));
        }
        handle_msg.line("\t} => {");
        handle_msg.line(format!("\t\tself.project.add_{}(ptr, {} {{", obj_type.name.to_case(Case::Snake), obj_type.name));
        handle_msg.line(format!("\t\t\t{}: Register::from_update(parent.clone(), 0),", obj_type.parent.to_case(Case::Snake)));
        for field in obj_type.fields {
            handle_msg.line(format!("\t\t\t{0}: Register::from_update({0}.clone(), 0),", field.name));
        }
        for child in obj_type.children {
            let child_obj = find_obj_type(&child);
            handle_msg.line(format!("\t\t\t{}: ChildList::new(),", child_obj.list_name));
        }
        handle_msg.line("\t\t});");
        handle_msg.line("\t\tself.serializer.set_obj_data(&self.project, ptr);");
        handle_msg.line("\t\tself.serializer.set_obj_data(&self.project, parent.value.0);");
        handle_msg.line(format!("\t\tself.broadcast(Message::Obj(ObjMessage::Add{} {{", obj_type.name));
        handle_msg.line("\t\t\tptr,");
        handle_msg.line("\t\t\tparent,");
        for field in obj_type.fields {
            handle_msg.line(format!("\t\t\t{},", field.name));
        }
        handle_msg.line("\t\t}), Some(client_id));");
        handle_msg.line("\t\tSome(())");
        handle_msg.line("\t},");

        // Delete
        handle_msg.line(format!("\tObjMessage::Delete{} {{ ptr }} => {{", obj_type.name));
        handle_msg.line(format!("\t\tself.serializer.delete_{}(&self.project, ptr);", obj_type.name.to_case(Case::Snake)));
        handle_msg.line(format!("\t\tself.project.delete_{}(ptr)?;", obj_type.name.to_case(Case::Snake)));
        handle_msg.line(format!("\t\tself.broadcast(Message::Obj(ObjMessage::Delete{} {{ ptr }}), Some(client_id));", obj_type.name));
        handle_msg.line("\t\tSome(())");
        handle_msg.line("\t},");

        // Set
        for field in obj_type.fields {
            handle_msg.line(format!("\tObjMessage::Set{}{} {{ ptr, update }} => {{", obj_type.name, field.name.to_case(Case::Pascal)));
            handle_msg.line(format!("\t\tlet obj = self.project.{}.get_mut(ptr)?;", obj_type.list_name));
            handle_msg.line(format!("\t\tobj.{}.apply(update.clone());", field.name));
            handle_msg.line("\t\tself.serializer.set_obj_data(&self.project, ptr);");
            handle_msg.line(format!("\t\tself.broadcast(Message::Obj(ObjMessage::Set{}{} {{ ptr, update }}), Some(client_id));", obj_type.name, field.name.to_case(Case::Pascal)));
            handle_msg.line("\t\tSome(())");
            handle_msg.line("\t},");
        }

        // Transfer
        if (obj_type.flags & ObjTypeFlags::NoTransferGen).is_empty() {
            let parent = find_obj_type(obj_type.parent);
            handle_msg.line(format!("\tObjMessage::Transfer{} {{ ptr, parent_update }} => {{", obj_type.name));
            handle_msg.line(format!("\t\tself.project.{}.get(parent_update.value.0)?;", parent.list_name));
            handle_msg.line(format!("\t\tlet obj = self.project.{}.get_mut(ptr)?;", obj_type.list_name));
            handle_msg.line(format!("\t\tlet old_parent = obj.{}.0;", obj_type.parent.to_case(Case::Snake)));
            handle_msg.line(format!("\t\tif obj.{}.apply(parent_update.clone()) {{", obj_type.parent.to_case(Case::Snake)));
            handle_msg.line(format!("\t\t\tself.project.{}.get_mut(old_parent)?.{}.remove(ptr);", parent.list_name, obj_type.list_name));
            handle_msg.line(format!("\t\t\tself.project.{}.get_mut(parent_update.value.0)?.{}.insert(parent_update.value.1.clone(), ptr);", parent.list_name, obj_type.list_name));
            handle_msg.line("\t\t}");
            handle_msg.line("\t\tself.serializer.set_obj_data(&self.project, ptr);");
            handle_msg.line("\t\tself.serializer.set_obj_data(&self.project, old_parent);");
            handle_msg.line("\t\tself.serializer.set_obj_data(&self.project, parent_update.value.0);");
            handle_msg.line(format!("\t\tself.broadcast(Message::Obj(ObjMessage::Transfer{} {{ ptr, parent_update }}), Some(client_id));", obj_type.name));
            handle_msg.line("\t\tSome(())");
            handle_msg.line("\t},");
        }
    }
    handle_msg.line("\t_ => {Some(())}");
    handle_msg.line("}");

    // Handle load request
    let handle_load = project_server_impl.new_fn("handle_load_request")
        .arg_mut_self()
        .arg("client_id", "u64")
        .arg("req", "LoadRequest")
        .ret("Option<()>");
    handle_load.line("match req {");
    for obj_type in &OBJ_TYPES {
        if !obj_type.is_asset() {
            continue;
        }
        handle_load.line(format!("\tLoadRequest::{}(ptr) => {{", obj_type.name));
        handle_load.line(format!("\t\tlet obj = self.project.{}.get(ptr)?;", obj_type.list_name));
        handle_load.line(format!("\t\tself.send(Message::LoadResult(LoadResult::{}(", obj_type.name));
        handle_load.line("\t\t\tptr,");
        for child in obj_type.children {
            let child_obj = find_obj_type(child);
            handle_load.line(format!("\t\t\tobj.{0}.iter_ref(&self.project.{0}).map(|child| self.get_{1}_load_data(&child, child.ptr())).collect(),", child_obj.list_name, child_obj.name.to_case(Case::Snake)));
        }
        handle_load.line("\t\t)), client_id);");
        handle_load.line("\t},");
    }
    handle_load.line("}");
    handle_load.line("Some(())");

    for obj_type in &OBJ_TYPES {
        let get_load_data_fn = project_server_impl.new_fn(format!("get_{}_load_data", obj_type.name.to_case(Case::Snake)).as_str())
            .arg_ref_self()
            .arg("obj", format!("&{}", obj_type.name))
            .arg("ptr", format!("ObjPtr<{}>", obj_type.name))
            .ret(format!("{}LoadData", obj_type.name));
        get_load_data_fn.line(format!("{}LoadData {{", obj_type.name));
        get_load_data_fn.line("\tptr,");
        get_load_data_fn.line(format!("\tparent: obj.{}.to_update(),", obj_type.parent.to_case(Case::Snake)));
        for field in obj_type.fields {
            get_load_data_fn.line(format!("\t{0}: obj.{0}.to_update(),", field.name));
        }
        for child in obj_type.children {
            let child_obj = find_obj_type(&child);
            get_load_data_fn.line(format!("\t{0}: obj.{0}.iter_ref(&self.project.{0}).map(|child| self.get_{1}_load_data(&child, child.ptr())).collect(),", child_obj.list_name, child_obj.name.to_case(Case::Snake)));
        }
        get_load_data_fn.line("}");
    }

    // Get welcome data
    for obj_type in &OBJ_TYPES {
        if !obj_type.is_asset() {
            continue;
        }
        let get_welcome_data = project_server_impl.new_fn(format!("get_{}_data", obj_type.name.to_case(Case::Snake)).as_str())
            .arg_ref_self()
            .arg("ptr", format!("ObjPtr<{}>", obj_type.name))
            .ret(format!("Welcome{}Data", obj_type.name.to_case(Case::Pascal)));

        get_welcome_data.line(format!("let obj = self.project.{}.get(ptr).unwrap();", obj_type.list_name));
        get_welcome_data.line(format!("Welcome{}Data {{", obj_type.name));
        get_welcome_data.line("\tptr,");
        get_welcome_data.line(format!("\tparent: obj.{}.to_update(),", obj_type.parent.to_case(Case::Snake)));
        for field in obj_type.fields {
            get_welcome_data.line(format!("\t{0}: obj.{0}.to_update(),", field.name));
        }
        get_welcome_data.line("}");
    }

    let _ = std::fs::write("src/server/server.gen.rs", scope.to_string());
}
