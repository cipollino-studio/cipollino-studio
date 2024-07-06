
use codegen::{Field, Scope};
use convert_case::{Case, Casing};
use bitflags::bitflags;

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

static OBJ_TYPES: [ObjType; 2] = [
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

fn find_obj_type(name: &'static str) -> &'static ObjType {
    for obj_type in &OBJ_TYPES {
        if obj_type.name == name {
            return obj_type;
        }
    }
    panic!("obj type {} not found", name);
}

// src/project/OBJ.gen.rs
fn generate_obj_code() {
    for obj_type in &OBJ_TYPES {
        let parent_obj_type = find_obj_type(obj_type.parent);

        let mut scope = Scope::new();

        scope.import("crate::project", "Project");
        scope.import("crate::project::obj", "ObjPtr");
        if obj_type.parent != obj_type.name {
            scope.import(format!("crate::project::{}", obj_type.parent.to_case(Case::Snake)).as_str(), &obj_type.parent);
        }
        for child in obj_type.children {
            if *child != obj_type.name {
                scope.import(format!("crate::project::{}", child.to_case(Case::Snake)).as_str(), &child);
            }
        }

        // Generate object struct definition
        scope.import("crate::crdt::register", "Register");
        scope.import("crate::crdt::fractional_index", "FractionalIndex");
        scope.import("crate::project::obj", "ChildList");

        let obj_struct = scope.new_struct(&obj_type.name).vis("pub");

        let parent_type = format!("Register<(ObjPtr<{}>, FractionalIndex)>", obj_type.parent);
        let mut parent_field = Field::new(&obj_type.parent.to_case(Case::Snake), parent_type);
        parent_field.vis("pub");
        obj_struct.push_field(parent_field);

        for child in obj_type.children {
            let child_obj = find_obj_type(child);
            let mut child_field = Field::new(&child_obj.list_name, format!("ChildList<{}>", child));
            child_field.vis("pub");
            obj_struct.push_field(child_field);
        }

        for field in obj_type.fields {
            let mut struct_field = Field::new(&field.name, format!("Register<{}>", field.ty));
            struct_field.vis("pub");
            obj_struct.push_field(struct_field);
        }

        // Generate ObjSerialize implementation
        scope.import("crate::serialization", "ObjSerialize");
        scope.import("crate::serialization", "Serializer");

        let obj_serialize_impl = scope.new_impl(&obj_type.name).impl_trait("ObjSerialize");

        let serialize_fn = obj_serialize_impl.new_fn("obj_serialize")
            .arg_ref_self()
            .arg("project", "&Project")
            .arg("serializer", "&mut Serializer")
            .ret("bson::Bson")
            .line("bson::bson!({");
        for field in obj_type.fields {
            serialize_fn.line(format!("\t\"{}\": self.{}.value.obj_serialize(project, serializer),", field.name, field.name));
        }
        serialize_fn.line(format!("\t\"{}\": self.{}.0.obj_serialize(project, serializer),", obj_type.parent.to_case(Case::Snake), obj_type.parent.to_case(Case::Snake)));
        for child in obj_type.children {
            let list_name = find_obj_type(child).list_name;
            serialize_fn.line(format!("\t\"{}\": self.{}.obj_serialize(project, serializer),", list_name, list_name));
        }
        serialize_fn.line("})");

        let deserialize_fn = obj_serialize_impl.new_fn("obj_deserialize")
            .arg("project", "&mut Project")
            .arg("data", "&bson::Bson")
            .arg("serializer", "&mut Serializer")
            .arg("idx", "FractionalIndex")
            .ret("Option<Self>");
        deserialize_fn.line("let data = data.as_document()?;");
        deserialize_fn.line(format!("let parent_ptr = data.get(\"{}\").map(|parent| ObjPtr::obj_deserialize(project, parent, serializer, idx.clone())).flatten().unwrap_or(ObjPtr::null());", obj_type.parent.to_case(Case::Snake)));

        deserialize_fn.line(format!("Some({} {{", obj_type.name));
        deserialize_fn.line(format!("\t{}: Register::new((parent_ptr, idx.clone()), 0),", obj_type.parent.to_case(Case::Lower)));
        for field in obj_type.fields {
            deserialize_fn.line(format!("\t{}: data.get(\"{}\").map(|val| Register::obj_deserialize(project, val, serializer, idx.clone())).flatten().unwrap_or(Register::new({}, 0)),", field.name, field.name, field.default));
        }
        for child in obj_type.children {
            let child_obj = find_obj_type(child);
            deserialize_fn.line(format!("\t{}: data.get(\"{}\").map(|val| ChildList::obj_deserialize(project, val, serializer, idx.clone())).flatten().unwrap_or(ChildList::new()),", child_obj.list_name, child_obj.list_name));
        }
        deserialize_fn.line("})");

        // Object Tree Manipulation
        let project_impl = scope.new_impl("Project");

        let add_fn = project_impl.new_fn(format!("add_{}", obj_type.name.to_case(Case::Snake)).as_str())
            .arg_mut_self()
            .arg("new_obj_ptr", format!("ObjPtr<{}>", obj_type.name))
            .arg("obj", obj_type.name)
            .ret("Option<()>")
            .vis("pub(crate)");
        add_fn.line(format!("let parent_ptr = obj.{}.0;", obj_type.parent.to_case(Case::Snake)));
        add_fn.line(format!("let idx = obj.{}.1.clone();", obj_type.parent.to_case(Case::Snake)));
        add_fn.line(format!("self.{}.objs.insert(new_obj_ptr, obj);", obj_type.list_name));
        add_fn.line(format!("let list_in_parent = &mut self.{}.get_mut(parent_ptr)?.{};", parent_obj_type.list_name, obj_type.list_name));
        add_fn.line("list_in_parent.insert(idx.clone(), new_obj_ptr);");
        add_fn.line("Some(())");
        
        let _ = std::fs::write(format!("src/project/{}.gen.rs", obj_type.name.to_case(Case::Snake)), scope.to_string());
    }

}

// src/protocol.gen.rs
fn generate_protocol_code() {

    let mut scope = Scope::new();

    for obj_type in &OBJ_TYPES {
        scope.import(format!("crate::project::{}", obj_type.name.to_case(Case::Snake)).as_str(), &obj_type.name);
    }
    
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

// src/client/collab.gen.rs
fn generate_client_collab_code() {

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

// src/client/client.gen.rs
fn generate_client_obj_code() {

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

// src/project/action.gen.rs
fn generate_action_code() {

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

// src/server/server.gen.rs
fn generate_server_code() {
    let mut scope = Scope::new();

    for obj_type in &OBJ_TYPES {
        if obj_type.is_asset() {
            scope.import("crate::protocol", format!("Welcome{}Data", obj_type.name).as_str());
        }
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
            handle_msg.line(format!("\t\t{}", field.name));
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

fn main() {

    generate_obj_code();
    generate_protocol_code();
    generate_client_collab_code();
    generate_client_obj_code();
    generate_action_code();
    generate_server_code();

}
