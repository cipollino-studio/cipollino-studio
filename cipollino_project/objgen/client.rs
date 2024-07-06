use codegen::Scope;
use convert_case::{Case, Casing};

use super::{find_obj_type, ObjTypeFlags, OBJ_TYPES};

// src/client/client.gen.rs
pub fn generate_client_code() {

    let mut scope = Scope::new();

    scope.import("crate::project::obj", "ObjPtr");
    scope.import("crate::crdt::register", "Register");
    scope.import("crate::project::obj", "ChildList");
    scope.import("crate::protocol", "Message");
    scope.import("crate::protocol", "ObjMessage");
    scope.import("crate::project::action", "Action");
    scope.import("crate::project::action", "ObjAction");
    scope.import("crate::crdt::fractional_index", "FractionalIndex");

    for obj_type in &OBJ_TYPES {

        scope.import(format!("crate::project::{}", obj_type.name.to_case(Case::Snake)).as_str(), &obj_type.name);

        let client_impl = scope.new_impl("ProjectClient");

        // Add
        let add_fn = client_impl.new_fn(format!("add_{}", obj_type.name.to_case(Case::Snake)).as_str())
            .vis("pub")
            .arg_mut_self()
            .arg("project", "&mut Project")
            .arg("parent", format!("ObjPtr<{}>", obj_type.parent))
            .arg("idx", "FractionalIndex")
            .ret("Option<()>");

        for field in obj_type.fields {
            add_fn.arg(field.name, field.ty);
        }
        add_fn.line("let ptr = ObjPtr::from_key(self.next_key()?);");
        for field in obj_type.fields { 
            add_fn.line(format!("let {0} = Register::new({0}, self.client_id());", field.name));
            add_fn.line(format!("let {0}_update = {0}.to_update();", field.name));
        }
        add_fn.line("let parent_reg = Register::new((parent, idx), self.client_id());");
        add_fn.line("let parent_update = parent_reg.to_update();");
        add_fn.line(format!("project.add_{}(ptr, {} {{", obj_type.name.to_case(Case::Snake), obj_type.name));
        add_fn.line(format!("\t{}: parent_reg,", obj_type.parent.to_case(Case::Snake)));
        for field in obj_type.fields {
            add_fn.line(format!("\t{},", field.name));
        }
        for child in obj_type.children {
            let child_obj = find_obj_type(child);
            add_fn.line(format!("\t{}: ChildList::new(),", child_obj.list_name));
        }
        add_fn.line("})?;");

        add_fn.line("match self {");
        add_fn.line("\tProjectClient::Local(local) => {");
        add_fn.line("\t\tlocal.serializer.set_obj_data(project, ptr);");
        add_fn.line("\t\tlocal.serializer.set_obj_data(project, parent);");
        add_fn.line("\t\tlocal.update_root_obj(project);");
        add_fn.line("\t},");
        add_fn.line("\tProjectClient::Collab(collab) => {");
        add_fn.line(format!("\t\tcollab.socket.send(Message::Obj(ObjMessage::Add{} {{", obj_type.name));
        add_fn.line("\t\t\tptr,");
        for field in obj_type.fields {
            add_fn.line(format!("\t\t\t{0}: {0}_update,", field.name));
        }
        add_fn.line("\t\t\tparent: parent_update,");
        add_fn.line("\t\t}))");
        add_fn.line("\t}");
        add_fn.line("}");
        add_fn.line("Some(())");

        // Set
        for field in obj_type.fields {
            let set_no_action_fn = client_impl.new_fn(format!("set_{}_{}_no_action", obj_type.name.to_case(Case::Snake), field.name).as_str())
                .vis("pub")
                .arg_mut_self()
                .arg("project", "&mut Project")
                .arg("ptr", format!("ObjPtr<{}>", obj_type.name))
                .arg(field.name, field.ty)
                .ret("Option<()>");
            set_no_action_fn.line(format!("let obj = project.{}.get_mut(ptr)?;", obj_type.list_name));
            set_no_action_fn.line(format!("let update = obj.{0}.set({0})?;", field.name));
            set_no_action_fn.line("match self {");
            set_no_action_fn.line("\tProjectClient::Local(local) => {");
            set_no_action_fn.line("\t\tlocal.serializer.set_obj_data(project, ptr);");
            set_no_action_fn.line("\t},");
            set_no_action_fn.line("\tProjectClient::Collab(collab) => {");
            set_no_action_fn.line(format!("\t\tcollab.socket.send(Message::Obj(ObjMessage::Set{}{} {{", obj_type.name, field.name.to_case(Case::Pascal))); 
            set_no_action_fn.line("\t\t\tptr,");
            set_no_action_fn.line("\t\t\tupdate,");
            set_no_action_fn.line("\t\t}));");
            set_no_action_fn.line("\t},");
            set_no_action_fn.line("}");
            set_no_action_fn.line("Some(())");

            let set_fn = client_impl.new_fn(format!("set_{}_{}", obj_type.name.to_case(Case::Snake), field.name).as_str())
                .vis("pub")
                .arg_mut_self()
                .arg("project", "&mut Project")
                .arg("ptr", format!("ObjPtr<{}>", obj_type.name))
                .arg(field.name, field.ty)
                .arg("action", "&mut Action")
                .ret("Option<()>");

            set_fn.line(format!("let old_val = project.{}.get(ptr)?.{}.value.clone();", obj_type.list_name, field.name));
            set_fn.line(format!("self.set_{0}_{1}_no_action(project, ptr, {1}.clone());", obj_type.name.to_case(Case::Snake), field.name));
            set_fn.line(format!("action.add_act(ObjAction::Set{}{}(ptr, old_val));", obj_type.name, field.name.to_case(Case::Pascal)));
            set_fn.line("Some(())");

        }

        // Transfer
        let parent = find_obj_type(obj_type.parent);
        let transfer_no_action_fn = client_impl.new_fn(format!("transfer_{}_no_action{}", obj_type.name.to_case(Case::Snake), if !(obj_type.flags & ObjTypeFlags::NoTransferGen).is_empty() { "_gen" } else { "" } ).as_str())
            .vis("pub")
            .arg_mut_self()
            .arg("project", "&mut Project")
            .arg("ptr", format!("ObjPtr<{}>", obj_type.name))
            .arg("new_parent_ptr", format!("ObjPtr<{}>", obj_type.parent))
            .arg("idx", "FractionalIndex")
            .ret("Option<()>");

        transfer_no_action_fn.line(format!("project.{}.get(new_parent_ptr)?;", parent.list_name));
        transfer_no_action_fn.line(format!("let obj = project.{}.get_mut(ptr)?;", obj_type.list_name));
        transfer_no_action_fn.line(format!("let old_parent = obj.{}.0;", obj_type.parent.to_case(Case::Snake)));
        transfer_no_action_fn.line(format!("let update = obj.{}.set((new_parent_ptr, idx.clone()))?;", obj_type.parent.to_case(Case::Snake)));
        transfer_no_action_fn.line(format!("project.{}.get_mut(old_parent)?.{}.remove(ptr);", parent.list_name, obj_type.list_name));
        transfer_no_action_fn.line(format!("let new_parent = project.{}.get_mut(new_parent_ptr)?;", parent.list_name));
        transfer_no_action_fn.line(format!("new_parent.{}.insert(idx, ptr);", obj_type.list_name));
        transfer_no_action_fn.line("match self {");
        transfer_no_action_fn.line("\tProjectClient::Local(local) => {");
        transfer_no_action_fn.line("\t\tlocal.serializer.set_obj_data(project, ptr);");
        transfer_no_action_fn.line("\t\tlocal.serializer.set_obj_data(project, old_parent);");
        transfer_no_action_fn.line("\t\tlocal.serializer.set_obj_data(project, new_parent_ptr);");
        transfer_no_action_fn.line("\t},");
        transfer_no_action_fn.line("\tProjectClient::Collab(collab) => {");
        transfer_no_action_fn.line(format!("\t\tcollab.socket.send(Message::Obj(ObjMessage::Transfer{} {{", obj_type.name));
        transfer_no_action_fn.line("\t\t\tptr,");
        transfer_no_action_fn.line("\t\t\tparent_update: update,");
        transfer_no_action_fn.line("\t\t}));");
        transfer_no_action_fn.line("\t},");
        transfer_no_action_fn.line("};");
        transfer_no_action_fn.line("Some(())");

        let transfer_fn = client_impl.new_fn(format!("transfer_{}", obj_type.name.to_case(Case::Snake)).as_str())
            .vis("pub")
            .arg_mut_self()
            .arg("project", "&mut Project")
            .arg("ptr", format!("ObjPtr<{}>", obj_type.name))
            .arg("new_parent_ptr", format!("ObjPtr<{}>", obj_type.parent))
            .ret("Option<()>");

        transfer_fn.arg("idx", "FractionalIndex");

        transfer_fn.arg("action", "&mut Action");

        transfer_fn.line(format!("let old_parent = project.{}.get(ptr)?.{}.value.0;", obj_type.list_name, obj_type.parent.to_case(Case::Snake)));
        transfer_fn.line(format!("let old_idx = project.{}.get(ptr)?.{}.value.1.clone();", obj_type.list_name, obj_type.parent.to_case(Case::Snake)));
        transfer_fn.line(format!("self.transfer_{}_no_action(project, ptr, new_parent_ptr, idx);", obj_type.name.to_case(Case::Snake)));
        transfer_fn.line(format!("action.add_act(ObjAction::Transfer{}(ptr, old_parent, old_idx));", obj_type.name));
        transfer_fn.line("Some(())");

    }

    let _ = std::fs::write("src/client/client.gen.rs", scope.to_string());

}
