use codegen::Scope;
use convert_case::{Case, Casing};

use super::{find_obj_type, OBJ_TYPES};


// src/project/action.gen.rs
pub fn generate_action_code() {

    let mut scope = Scope::new();

    scope.import("crate::crdt::fractional_index", "FractionalIndex");
    for obj_type in &OBJ_TYPES {
        scope.import(format!("crate::project::{}", obj_type.name.to_case(Case::Snake)).as_str(), &obj_type.name);
        scope.import(format!("crate::project::{}", obj_type.name.to_case(Case::Snake)).as_str(), format!("{}RawData", &obj_type.name).as_str());
    }

    // ObjAction enum
    let obj_action_enum = scope.new_enum("ObjAction")
        .vis("pub");

    for obj_type in &OBJ_TYPES {

        // Add
        obj_action_enum.new_variant(format!("Add{}", obj_type.name))
            .tuple(format!("{}RawData", obj_type.name).as_str())
            .tuple(format!("ObjPtr<{}>", obj_type.parent).as_str())
            .tuple("FractionalIndex");

        // Delete
        obj_action_enum.new_variant(format!("Delete{}", obj_type.name))
            .tuple(format!("ObjPtr<{}>", obj_type.name).as_str());

        // Set
        for field in obj_type.fields {
            obj_action_enum.new_variant(format!("Set{}{}", obj_type.name, field.name.to_case(Case::Pascal)))
                .tuple(format!("ObjPtr<{}>", obj_type.name).as_str())
                .tuple(&field.ty);
        }

        // Transfer
        obj_action_enum.new_variant(format!("Transfer{}", obj_type.name))
            .tuple(format!("ObjPtr<{}>", obj_type.name).as_str())
            .tuple(format!("ObjPtr<{}>", obj_type.parent).as_str())
            .tuple("FractionalIndex");

    }

    let obj_action_impl = scope.new_impl("ObjAction");
    
    let perform = obj_action_impl.new_fn("perform")
        .arg_self()
        .arg("project", "&mut Project")
        .arg("client", "&mut ProjectClient")
        .ret("Option<Self>");

    perform.line("match self {");
    for obj_type in &OBJ_TYPES {

        // Add
        perform.line(format!("\tObjAction::Add{}(data, parent, idx) => {{", obj_type.name));
        perform.line("\t\tlet ptr = data.ptr;");
        perform.line(format!("\t\tclient.recreate_{}(project, parent, idx, data)?;", obj_type.name.to_case(Case::Snake)));
        perform.line(format!("\t\tSome(ObjAction::Delete{}(ptr))", obj_type.name));
        perform.line("\t},");

        // Delete
        let parent_obj = find_obj_type(obj_type.parent);
        perform.line(format!("\tObjAction::Delete{}(ptr) => {{", obj_type.name));
        perform.line(format!("\t\tlet obj = project.{}.get(ptr)?;", obj_type.list_name));
        perform.line(format!("\t\tlet parent = obj.{}.0;", parent_obj.name.to_case(Case::Snake)));
        perform.line(format!("\t\tlet idx = obj.{}.1.clone();", parent_obj.name.to_case(Case::Snake)));
        perform.line(format!("\t\tlet data = project.get_{}_raw_data(project.{}.get(ptr)?, ptr);", obj_type.name.to_case(Case::Snake), obj_type.list_name));
        perform.line(format!("\t\tclient.delete_{}_no_action(project, ptr);", obj_type.name.to_case(Case::Snake)));
        perform.line(format!("\t\tSome(ObjAction::Add{}(data, parent, idx))", obj_type.name));
        perform.line("\t},");

        // Set
        for field in obj_type.fields {
            perform.line(format!("\tObjAction::Set{}{}(ptr, new_value) => {{", obj_type.name, field.name.to_case(Case::Pascal)));
            perform.line(format!("\t\tlet old_value = project.{}.get(ptr)?.{}.value().clone();", obj_type.list_name, field.name));
            perform.line(format!("\t\tclient.set_{}_{}_no_action(project, ptr, new_value.clone())?;", obj_type.name.to_case(Case::Snake), field.name));
            perform.line(format!("\t\tSome(ObjAction::Set{}{}(ptr, old_value))", obj_type.name, field.name.to_case(Case::Pascal)));
            perform.line("\t},");
        }

        // Transfer
        perform.line(format!("\tObjAction::Transfer{}(ptr, new_parent, new_idx) => {{", obj_type.name));
        perform.line(format!("\t\tlet old_parent = project.{}.get(ptr)?.{}.value().0;", obj_type.list_name, obj_type.parent.to_case(Case::Snake)));
        perform.line(format!("\t\tlet old_idx = project.{}.get(ptr)?.{}.value().1.clone();", obj_type.list_name, obj_type.parent.to_case(Case::Snake)));
        perform.line(format!("\t\tclient.transfer_{}_no_action(project, ptr, new_parent, new_idx.clone());", obj_type.name.to_case(Case::Snake)));
        perform.line(format!("\t\tSome(ObjAction::Transfer{}(ptr, old_parent, old_idx))", obj_type.name));
        perform.line("\t},");
    }
    perform.line("}");

    let _ = std::fs::write("src/project/action.gen.rs", scope.to_string());

}
