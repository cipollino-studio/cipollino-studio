
use std::collections::HashSet;

use crate::{Action, Project};

mod operations;
pub(crate) use operations::*;

mod folder;
pub use folder::*;

mod clip;
pub use clip::*;

mod palette;
pub use palette::*;

mod audio;
pub use audio::*;

pub trait Asset: alisa::TreeObj<ParentPtr = alisa::Ptr<Folder>, Project = Project, ChildList = alisa::UnorderedChildList<alisa::LoadingPtr<Self>>> {

    const NAME: &'static str;

    fn name(&self) -> &String;    
    fn name_mut(&mut self) -> &mut String;

    fn rename(action: &mut Action, ptr: alisa::Ptr<Self>, name: String);
    fn delete(action: &mut Action, ptr: alisa::Ptr<Self>);

    fn get_sibling_names(child_list: &Self::ChildList, recorder: &alisa::Recorder<Project>, exclude: Option<alisa::Ptr<Self>>) -> HashSet<String> {
        child_list.iter()
            .filter(|ptr| Some(ptr.ptr()) != exclude)
            .filter_map(|ptr| recorder.get_obj(ptr.ptr())).map(|asset| asset.name().clone())
            .collect()
    }

}
