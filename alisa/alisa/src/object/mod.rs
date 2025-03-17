
use crate::{Project, Serializable};

mod ptr;
pub use ptr::*;

mod obj_list;
pub use obj_list::*;

mod obj_kind;
pub use obj_kind::*;

pub trait Object: Sized + Clone + Serializable<Self::Project> + 'static + Send + Sync{

    type Project: Project;

    const NAME: &'static str;

    fn list(objects: &<Self::Project as Project>::Objects) -> &ObjList<Self>;
    fn list_mut(objects: &mut <Self::Project as Project>::Objects) -> &mut ObjList<Self>;

}
