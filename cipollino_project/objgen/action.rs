use codegen::Scope;
use convert_case::{Case, Casing};

use super::OBJ_TYPES;


// src/project/action.gen.rs
pub fn generate_action_code() {

    let mut scope = Scope::new();

    scope.import("crate::crdt::fractional_index", "FractionalIndex");
    for obj_type in &OBJ_TYPES {
        scope.import(format!("crate::project::{}", obj_type.name.to_case(Case::Snake)).as_str(), &obj_type.name);
    }

    // ObjAction enum
    let obj_action_enum = scope.new_enum("ObjAction")
        .vis("pub");

    for obj_type in &OBJ_TYPES {

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
    
    // Redo
    let redo_fn = obj_action_impl.new_fn("redo")
        .arg_ref_self()
        .arg("project", "&mut Project")
        .arg("client", "&mut ProjectClient")
        .ret("Option<Self>");

    redo_fn.line("match self {");
    for obj_type in &OBJ_TYPES {
        // Set
        for field in obj_type.fields {
            redo_fn.line(format!("\tObjAction::Set{}{}(ptr, new_value) => {{", obj_type.name, field.name.to_case(Case::Pascal)));
            redo_fn.line(format!("\t\tlet old_value = project.{}.get(*ptr)?.{}.value().clone();", obj_type.list_name, field.name));
            redo_fn.line(format!("\t\tclient.set_{}_{}_no_action(project, *ptr, new_value.clone())?;", obj_type.name.to_case(Case::Snake), field.name));
            redo_fn.line(format!("\t\tSome(ObjAction::Set{}{}(*ptr, old_value))", obj_type.name, field.name.to_case(Case::Pascal)));
            redo_fn.line("\t},");
        }

        // Transfer
        redo_fn.line(format!("\tObjAction::Transfer{}(ptr, new_parent, new_idx) => {{", obj_type.name));
        redo_fn.line(format!("\t\tlet old_parent = project.{}.get(*ptr)?.{}.value().0;", obj_type.list_name, obj_type.parent.to_case(Case::Snake)));
        redo_fn.line(format!("\t\tlet old_idx = project.{}.get(*ptr)?.{}.value().1.clone();", obj_type.list_name, obj_type.parent.to_case(Case::Snake)));
        redo_fn.line(format!("\t\tclient.transfer_{}_no_action(project, *ptr, *new_parent, new_idx.clone());", obj_type.name.to_case(Case::Snake)));
        redo_fn.line(format!("\t\tSome(ObjAction::Transfer{}(*ptr, old_parent, old_idx))", obj_type.name));
        redo_fn.line("\t},");
    }
    redo_fn.line("}");

    // Undo
    let undo_fn = obj_action_impl.new_fn("undo")
        .arg_ref_self()
        .arg("project", "&mut Project")
        .arg("client", "&mut ProjectClient")
        .ret("Option<Self>");

    undo_fn.line("match self {");
    for obj_type in &OBJ_TYPES {
        // Set
        for field in obj_type.fields {
            undo_fn.line(format!("\tObjAction::Set{}{}(ptr, old_value) => {{", obj_type.name, field.name.to_case(Case::Pascal)));
            undo_fn.line(format!("\t\tlet new_value = project.{}.get(*ptr)?.{}.value().clone();", obj_type.list_name, field.name));
            undo_fn.line(format!("\t\tclient.set_{}_{}_no_action(project, *ptr, old_value.clone())?;", obj_type.name.to_case(Case::Snake), field.name));
            undo_fn.line(format!("\t\tSome(ObjAction::Set{}{}(*ptr, new_value))", obj_type.name, field.name.to_case(Case::Pascal)));
            undo_fn.line("\t},");
        }

        // Transfer
        undo_fn.line(format!("\tObjAction::Transfer{}(ptr, old_parent, old_idx) => {{", obj_type.name));
        undo_fn.line(format!("\t\tlet new_parent = project.{}.get(*ptr)?.{}.value().0;", obj_type.list_name, obj_type.parent.to_case(Case::Snake)));
        undo_fn.line(format!("\t\tlet new_idx = project.{}.get(*ptr)?.{}.value().1.clone();", obj_type.list_name, obj_type.parent.to_case(Case::Snake)));
        undo_fn.line(format!("\t\tclient.transfer_{}_no_action(project, *ptr, *old_parent, old_idx.clone());", obj_type.name.to_case(Case::Snake)));
        undo_fn.line(format!("\t\tSome(ObjAction::Transfer{}(*ptr, new_parent, new_idx))", obj_type.name));
        undo_fn.line("\t},");
    }
    undo_fn.line("}"); 

    let _ = std::fs::write("src/project/action.gen.rs", scope.to_string());

}
