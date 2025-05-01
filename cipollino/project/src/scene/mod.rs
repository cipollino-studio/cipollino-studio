
mod stroke;
pub use stroke::*;

mod fill;
pub use fill::*;

use crate::{Frame, Project};

alisa::ptr_enum!(SceneObjPtr loading [Stroke, Fill] childof alisa::Ptr<Frame>, in Project);
