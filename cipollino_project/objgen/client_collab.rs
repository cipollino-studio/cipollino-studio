
use codegen::Scope;
use convert_case::{Case, Casing};
use super::{find_obj_type, OBJ_TYPES};

// src/client/collab.gen.rs
pub fn generate_client_collab_code() {

    let mut scope = Scope::new();

    scope.import("crate::protocol", "ObjMessage");
    scope.import("crate::protocol", "LoadResult");
    scope.import("std::collections", "HashSet");

    for obj_type in &OBJ_TYPES {
        scope.import(format!("crate::project::{}", obj_type.name.to_case(Case::Snake)).as_str(), &obj_type.name);
        scope.import("crate::protocol", format!("{}LoadData", obj_type.name).as_str());
    }

    // Loading info 
    let load_info = scope.new_struct("CollabLoadInfo")
        .vis("pub(crate)");
    for obj_type in &OBJ_TYPES {
        if obj_type.is_asset() {
            load_info.field(format!("pub(crate) sent_{}_load_msg", obj_type.name.to_case(Case::Snake)).as_str(), format!("HashSet<ObjPtr<{}>>", obj_type.name));
        }
    }

    let new_collab_load_info = scope.new_impl("CollabLoadInfo")
        .new_fn("new")
        .ret("Self");

    new_collab_load_info.line("Self {");
    for obj_type in &OBJ_TYPES {
        if obj_type.is_asset() {
            new_collab_load_info.line(format!("\tsent_{}_load_msg: HashSet::new()", obj_type.name.to_case(Case::Snake)));
        }
    }
    new_collab_load_info.line("}");


    let collab_impl = scope.new_impl("Collab"); 

    // Handle message
    let handle_fn = collab_impl.new_fn("handle_obj_msg")
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

        // Delete messages
        handle_fn.line(format!("\tObjMessage::Delete{} {{ ptr }} => {{", obj_type.name));
        handle_fn.line(format!("\t\tproject.delete_{}(ptr)", obj_type.name.to_case(Case::Snake)));
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
        handle_fn.line(format!("\t\t\tif project.{}.is_loaded(parent_update.value.0) {{", parent.list_name));
        handle_fn.line(format!("\t\t\t\tproject.{}.get_mut(parent_update.value.0)?.{}.insert(parent_update.value.1.clone(), ptr);", parent.list_name, obj_type.list_name));
        handle_fn.line("\t\t\t}");
        handle_fn.line("\t\t}");
        handle_fn.line("\t\tSome(())");
        handle_fn.line("\t},");

    }

    handle_fn.line("}");

    // Handle load result
    let load_fn = collab_impl.new_fn("handle_load_result")
        .arg_mut_self()
        .arg("load", "LoadResult")
        .arg("project", "&mut Project")
        .arg("client_id", "u64")
        .ret("Option<()>");
    load_fn.line("match load {");
    for obj_type in &OBJ_TYPES {
        if !obj_type.is_asset() {
            continue;
        }
        load_fn.line(format!("\tLoadResult::{}(", obj_type.name));
        load_fn.line("\t\tptr,");
        for child in obj_type.children {
            let child_obj = find_obj_type(&child);
            load_fn.line(format!("\t\t{},", child_obj.list_name));
        }
        load_fn.line("\t) => {");
        load_fn.line(format!("\t\tproject.{}.mark_as_loaded(ptr);", obj_type.list_name));
        for child in obj_type.children {
            let child_obj = find_obj_type(&child);
            load_fn.line(format!("\t\tfor child in {} {{", child_obj.list_name));
            load_fn.line(format!("\t\t\tSelf::add_{}_from_load_data(project, child, client_id);", child_obj.name.to_case(Case::Snake)));
            load_fn.line("\t\t}");
        }
        load_fn.line("\t},");
    }
    load_fn.line("}");
    load_fn.line("Some(())");

    for obj_type in &OBJ_TYPES {
        let add_from_load_fn = collab_impl.new_fn(format!("add_{}_from_load_data", obj_type.name.to_case(Case::Snake)).as_str())
            .arg("project", "&mut Project")
            .arg("data", format!("{}LoadData", obj_type.name))
            .arg("client_id", "u64");
        add_from_load_fn.line(format!("project.add_{}(data.ptr, {} {{", obj_type.name.to_case(Case::Snake), obj_type.name));
        add_from_load_fn.line(format!("\t{}: Register::from_update(data.parent, client_id),", obj_type.parent.to_case(Case::Snake)));
        for field in obj_type.fields {
            add_from_load_fn.line(format!("\t{0}: Register::from_update(data.{0}, client_id),", field.name));
        }
        for child in obj_type.children {
            let child_obj = find_obj_type(&child);
            add_from_load_fn.line(format!("\t{}: ChildList::new(),", child_obj.list_name));
        }
        add_from_load_fn.line("});");

        for child in obj_type.children {
            let child_obj = find_obj_type(child);
            add_from_load_fn.line(format!("for child in data.{} {{", child_obj.list_name));
            add_from_load_fn.line(format!("\tSelf::add_{}_from_load_data(project, child, client_id);", child_obj.name.to_case(Case::Snake)));
            add_from_load_fn.line("}");
        }
    }

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

        add_fn.line(format!("project.{}.objs.insert(data.ptr, ObjState::ToLoad({} {{", obj_type.list_name, obj_type.name));
        add_fn.line(format!("\t{}: Register::from_update(data.parent, client_id),", obj_type.parent.to_case(Case::Snake)));
        for field in obj_type.fields {
            add_fn.line(format!("\t{0}: Register::from_update(data.{0}, client_id),", field.name));
        }
        for child in obj_type.children {
            let child_obj = find_obj_type(&child);
            add_fn.line(format!("\t{}: ChildList::new(),", child_obj.list_name));
        }
        add_fn.line("}));");
        add_fn.line("data.ptr");
    }

    let _ = std::fs::write("src/client/collab.gen.rs", scope.to_string());

}