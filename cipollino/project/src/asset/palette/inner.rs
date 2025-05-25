
use crate::{Color, Objects, Project};

use super::Palette;


#[derive(alisa::Serializable, Clone)]
pub struct PaletteInner {
    pub palette: alisa::Ptr<Palette>,
    pub colors: alisa::UnorderedChildList<alisa::LoadingPtr<Color>>
}

impl Default for PaletteInner {

    fn default() -> Self {
        Self {
            palette: Default::default(),
            colors: Default::default()
        }
    }

}

impl alisa::Object for PaletteInner {
    type Project = Project;
    const TYPE_ID: u16 = 9;

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.palette_inners 
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.palette_inners
    }
}

#[derive(alisa::Serializable, Default)]
pub struct CreatePaletteInner {
    pub palette: alisa::Ptr<Palette>,
    pub inner: alisa::Ptr<PaletteInner>
}

impl alisa::Operation for CreatePaletteInner {
    type Project = Project;
    const NAME: &'static str = "CreatePaletteInner";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Self::Project>) -> bool {

        let Some(palette) = recorder.get_obj(self.palette) else {
            return false;
        };
        let old_inner = palette.inner;
        if !old_inner.is_null() && recorder.get_obj(old_inner).is_some() {
            return false;
        }

        recorder.add_obj(self.inner, PaletteInner {
            palette: self.palette,
            colors: Default::default(),
        });
        let Some(palette) = recorder.get_obj_mut(self.palette) else {
            return false;
        };
        palette.inner = self.inner;
        
        true
    }
}