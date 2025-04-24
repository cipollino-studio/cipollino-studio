
mod stroke;
pub use stroke::*;

use crate::{Frame, Project};

alisa::ptr_enum!(SceneChildPtr loading [Stroke] childof alisa::Ptr<Frame>, in Project);
