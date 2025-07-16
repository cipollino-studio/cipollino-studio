
mod stroke;
pub use stroke::*;

mod fill;
pub use fill::*;

use crate::{Frame, Project};

alisa::ptr_enum!(SceneObjPtr owning [Stroke, Fill] childof alisa::Ptr<Frame>, in Project);
