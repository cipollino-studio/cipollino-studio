
use super::{find_obj_type, OBJ_TYPES};
use codegen::{Field, Scope};
use convert_case::{Case, Casing};

// src/project/OBJ.gen.rs
pub fn generate_obj_code() {
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
                scope.import(format!("crate::project::{}", child.to_case(Case::Snake)).as_str(), format!("{}RawData", &child).as_str());
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

        // Raw data struct(used for add/delete undo & copy paste)
        let raw_data_struct = scope.new_struct(format!("{}RawData", obj_type.name).as_str())
            .vis("pub")
            .field("pub ptr", format!("ObjPtr<{}>", obj_type.name));
        for field in obj_type.fields {
            raw_data_struct.field(format!("pub {}", &field.name).as_str(), field.ty);
        } 
        for child in obj_type.children {
            let child_obj = find_obj_type(&child);
            raw_data_struct.field(format!("pub {}", &child_obj.list_name).as_str(), format!("Vec<{}RawData>", child_obj.name));
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


        scope.import("crate::project::obj", "ObjState");
        let project_impl = scope.new_impl("Project");

        // Add 
        let add_fn = project_impl.new_fn(format!("add_{}", obj_type.name.to_case(Case::Snake)).as_str())
            .arg_mut_self()
            .arg("new_obj_ptr", format!("ObjPtr<{}>", obj_type.name))
            .arg("obj", obj_type.name)
            .ret("Option<()>")
            .vis("pub(crate)");
        add_fn.line(format!("let parent_ptr = obj.{}.0;", obj_type.parent.to_case(Case::Snake)));
        add_fn.line(format!("if !self.{}.is_loaded(parent_ptr) {{", parent_obj_type.list_name));
        add_fn.line("\treturn None;");
        add_fn.line("}");
        add_fn.line(format!("let idx = obj.{}.1.clone();", obj_type.parent.to_case(Case::Snake)));
        add_fn.line(format!("let list_in_parent = &mut self.{}.get_mut(parent_ptr)?.{};", parent_obj_type.list_name, obj_type.list_name));
        add_fn.line("list_in_parent.insert(idx.clone(), new_obj_ptr);");
        add_fn.line(format!("self.{}.objs.insert(new_obj_ptr, ObjState::Loaded(obj));", obj_type.list_name));
        add_fn.line("Some(())");

        // Delete
        let delete_rec_fn = project_impl.new_fn(format!("delete_{}_rec", obj_type.name.to_case(Case::Snake)).as_str())
            .arg_mut_self()
            .arg("ptr", format!("ObjPtr<{}>", obj_type.name))
            .ret("Option<()>")
            .vis("pub(crate)");
        delete_rec_fn.line(format!("let obj = self.{}.remove(ptr)?;", obj_type.list_name));
        for child in obj_type.children {
            let child_obj = find_obj_type(&child);
            delete_rec_fn.line(format!("for (_idx, child) in obj.{0}.objs {{", child_obj.list_name));
            delete_rec_fn.line(format!("\tself.delete_{}(child);", child_obj.name.to_case(Case::Snake)));
            delete_rec_fn.line("}");
        }
        delete_rec_fn.line("Some(())");

        let delete_fn = project_impl.new_fn(format!("delete_{}", obj_type.name.to_case(Case::Snake)).as_str())
            .arg_mut_self()
            .arg("ptr", format!("ObjPtr<{}>", obj_type.name))
            .ret("Option<()>")
            .vis("pub(crate)");
        let parent_obj = find_obj_type(obj_type.parent);
        delete_fn.line(format!("let obj = self.{}.get(ptr)?;", obj_type.list_name));
        delete_fn.line(format!("let parent = self.{}.get_mut(obj.{}.0)?;", parent_obj.list_name, parent_obj.name.to_case(Case::Snake)));
        delete_fn.line(format!("parent.{}.remove(ptr);", obj_type.list_name));
        delete_fn.line(format!("self.delete_{}_rec(ptr)", obj_type.name.to_case(Case::Snake)));

        // Get raw data
        let get_raw_fn = project_impl.new_fn(format!("get_{}_raw_data", obj_type.name.to_case(Case::Snake)).as_str())
            .arg_ref_self()
            .arg("obj", format!("&{}", obj_type.name))
            .arg("ptr", format!("ObjPtr<{}>", obj_type.name))
            .ret(format!("{}RawData", obj_type.name))
            .vis("pub");
        get_raw_fn.line(format!("{}RawData {{", obj_type.name));
        get_raw_fn.line("\tptr,");
        for field in obj_type.fields {
            get_raw_fn.line(format!("\t{0}: obj.{0}.value.clone(),", field.name));
        }
        for child in obj_type.children {
            let child_obj = find_obj_type(&child);
            get_raw_fn.line(format!("\t{0}: obj.{0}.iter_ref(&self.{0}).map(|child| self.get_{1}_raw_data(&child, child.ptr())).collect(),", child_obj.list_name, child_obj.name.to_case(Case::Snake)));
        }
        get_raw_fn.line("}");

        // Delete from project file
        let serializer_impl = scope.new_impl("Serializer");
        let delete_fn = serializer_impl.new_fn(format!("delete_{}", obj_type.name.to_case(Case::Snake)).as_str())
            .arg_mut_self()
            .arg("project", "&Project")
            .arg("ptr", format!("ObjPtr<{}>", obj_type.name))
            .ret("Option<()>")
            .vis("pub(crate)");
        delete_fn.line(format!("let obj = project.{}.get(ptr)?;", obj_type.list_name));
        delete_fn.line("if let Some(ptr) = self.obj_ptrs.remove(&ptr.key) {");
        delete_fn.line("\tself.project_file.delete_obj(ptr);"); 
        delete_fn.line("}");
        for child in obj_type.children {
            let child_obj = find_obj_type(child);
            delete_fn.line(format!("for child in obj.{0}.iter_ref(&project.{0}) {{", child_obj.list_name));
            delete_fn.line(format!("\tself.delete_{}(project, child.ptr());", child_obj.name.to_case(Case::Snake)));
            delete_fn.line("}");
        }
        delete_fn.line("Some(())");

        let _ = std::fs::write(format!("src/project/{}.gen.rs", obj_type.name.to_case(Case::Snake)), scope.to_string());
    }

}
