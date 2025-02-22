
use std::collections::HashSet;

use crate::{Action, Client, Project};

mod operations;
pub(crate) use operations::*;

mod folder;
pub use folder::*;

mod clip;
pub use clip::*;

pub trait Asset: alisa::TreeObj<ParentPtr = alisa::Ptr<Folder>, Project = Project, ChildList = alisa::UnorderedChildList<Self>> {

    fn name(&self) -> &String;    
    fn name_mut(&mut self) -> &mut String;

    fn rename(client: &Client, action: &mut Action, ptr: alisa::Ptr<Self>, name: String);
    fn delete(client: &Client, action: &mut Action, ptr: alisa::Ptr<Self>);

    fn get_sibling_names(child_list: &Self::ChildList, objects: &alisa::ObjList<Self>, exclude: Option<alisa::Ptr<Self>>) -> HashSet<String> {
        child_list.iter()
            .filter(|ptr| Some(*ptr) != exclude)
            .filter_map(|ptr| objects.get(ptr)).map(|asset| asset.name().clone())
            .collect()
    }

}