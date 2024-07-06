
use codegen::Scope;
use convert_case::{Case, Casing};
use super::{find_obj_type, OBJ_TYPES};

// src/client/collab.gen.rs
pub fn generate_client_collab_code() {

    let mut scope = Scope::new();

    scope.import("crate::protocol", "ObjMessage");

    for obj_type in &OBJ_TYPES {
        scope.import(format!("crate::project::{}", obj_type.name.to_case(Case::Snake)).as_str(), &obj_type.name);
    }

    // Handle message
    let handle_fn = scope.new_impl("Collab")
        .new_fn("handle_obj_msg")
        .arg_mut_self()
        .arg("msg", "ObjMessage")
        .arg("project", "&mut Project")
        .ret("Option<()>");

    handle_fn.line("match msg {");

    for obj_type in &OBJ_TYPES {
        
        // Add message
        handle_fn.line(format!("\tObjMessage::Add{} {{", obj_type.name));
        handle_fn.line("\t\tptr,");
        handle_fn.line("\t\tparent,");
        for field in obj_type.fields {
            handle_fn.line(format!("\t\t{},", field.name));
        }
        handle_fn.line("\t} => {");
        handle_fn.line(format!("\t\tproject.add_{}(ptr, {} {{", obj_type.name.to_case(Case::Snake), obj_type.name));
        handle_fn.line(format!("\t\t\t{}: Register::from_update(parent, self.client_id),", obj_type.parent.to_case(Case::Snake)));
        for field in obj_type.fields {
            handle_fn.line(format!("\t\t\t{}: Register::from_update({}, self.client_id),", field.name, field.name));
        }
        for child in obj_type.children {
            let child_obj = find_obj_type(&child);
            handle_fn.line(format!("\t\t\t{}: ChildList::new(),", child_obj.list_name));
        }
        handle_fn.line("\t\t})");
        handle_fn.line("\t},");

        // Set messages
        for field in obj_type.fields {
            handle_fn.line(format!("\tObjMessage::Set{}{} {{ ptr, update }} => {{", obj_type.name, field.name.to_case(Case::Pascal)));
            handle_fn.line(format!("\t\tlet obj = project.{}.get_mut(ptr)?;", obj_type.list_name));
            handle_fn.line(format!("\t\tobj.{}.apply(update);", field.name));
            handle_fn.line("\t\tSome(())");
            handle_fn.line("\t},");
        }

        // Transfer message
        let parent = find_obj_type(obj_type.parent);
        handle_fn.line(format!("\tObjMessage::Transfer{} {{ ptr, parent_update }} => {{", obj_type.name));
        handle_fn.line(format!("\t\tproject.{}.get(parent_update.value.0)?;", parent.list_name));
        handle_fn.line(format!("\t\tlet obj = project.{}.get_mut(ptr)?;", obj_type.list_name));
        handle_fn.line(format!("\t\tlet old_parent = obj.{}.0;", obj_type.parent.to_case(Case::Snake)));
        handle_fn.line(format!("\t\tif obj.{}.apply(parent_update.clone()) {{", obj_type.parent.to_case(Case::Snake)));
        handle_fn.line(format!("\t\t\tproject.{}.get_mut(old_parent)?.{}.remove(ptr);", parent.list_name, obj_type.list_name));
        handle_fn.line(format!("\t\t\tproject.{}.get_mut(parent_update.value.0)?.{}.insert(parent_update.value.1.clone(), ptr);", parent.list_name, obj_type.list_name));
        handle_fn.line("\t\t}");
        handle_fn.line("\t\tSome(())");
        handle_fn.line("\t},");

    }

    handle_fn.line("}");

    // Add assets from welcome data
    for obj_type in &OBJ_TYPES {
        if obj_type.is_asset() {
            scope.import("crate::protocol", format!("Welcome{}Data", obj_type.name).as_str());
        }
    }

    let project_client_impl = scope.new_impl("ProjectClient");

    for obj_type in &OBJ_TYPES {
        if !obj_type.is_asset() {
            continue;
        }

        let add_fn = project_client_impl.new_fn(format!("add_{}_from_welcome_data", obj_type.name.to_case(Case::Snake)).as_str())
            .arg("project", "&mut Project")
            .arg("client_id", "u64")
            .arg("data", format!("Welcome{}Data", obj_type.name))
            .ret(format!("ObjPtr<{}>", obj_type.name)); 

        add_fn.line(format!("project.{}.objs.insert(data.ptr, {} {{", obj_type.list_name, obj_type.name));
        add_fn.line(format!("\t{}: Register::from_update(data.parent, client_id),", obj_type.parent.to_case(Case::Snake)));
        for field in obj_type.fields {
            add_fn.line(format!("\t{0}: Register::from_update(data.{0}, client_id),", field.name));
        }
        add_fn.line("});");
        add_fn.line("data.ptr");
    }

    let _ = std::fs::write("src/client/collab.gen.rs", scope.to_string());

}