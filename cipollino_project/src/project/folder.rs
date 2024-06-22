
use crate::{crdt::register::Register, serialization::{ObjSerialize, Serializer}};

use super::{obj::{ChildList, Obj, ObjList, ObjPtr}, Project};

pub struct Folder {
    pub parent: ObjPtr<Folder>,
    pub folders: ChildList<Folder>,
    pub name: Register<String> 
}

impl Obj for Folder {
    type Parent = ObjPtr<Folder>;

    fn obj_list(project: &Project) -> &ObjList<Self> {
        &project.folders
    }

    fn obj_list_mut(project: &mut Project) -> &mut ObjList<Self> {
        &mut project.folders
    }

    fn parent(&self) -> Self::Parent {
        self.parent
    }

    fn parent_mut(&mut self) -> &mut Self::Parent {
        &mut self.parent
    }

    fn list_in_parent(project: &Project, parent: Self::Parent) -> Option<&ChildList<Self>> {
        Some(&project.folders.get(parent)?.folders)
    }

    fn list_in_parent_mut(project: &mut Project, parent: Self::Parent) -> Option<&mut ChildList<Self>> {
        Some(&mut project.folders.get_mut(parent)?.folders)
    }

} 

impl ObjSerialize for Folder {

    fn obj_serialize(&self, project: &Project, serializer: &mut Serializer) -> bson::Bson {
        bson::bson!({
            "name": self.name.value.clone(),
            "parent": self.parent.obj_serialize(project, serializer),
            "folders": self.folders.obj_serialize(project, serializer)
        })
    }

    fn obj_deserialize(project: &mut Project, data: &bson::Bson, serializer: &mut Serializer) -> Option<Self> {
        Some(Folder {
            parent: data.as_document()
                .map(|doc| doc.get("parent")).flatten()
                .map(|parent| ObjPtr::obj_deserialize(project, parent, serializer)).flatten().unwrap_or(ObjPtr::null()),
            folders: data.as_document()
                .map(|doc| doc.get("folders")).flatten()
                .map(|folders| ChildList::obj_deserialize(project, folders, serializer)).flatten().unwrap_or(ChildList::new()),
            name: data.as_document()
                .map(|doc| doc.get("name")).flatten()
                .map(|name| Register::obj_deserialize(project, name, serializer)).flatten().unwrap_or(Register::new("Folder".to_owned(), 0)),
        })
    }

}
