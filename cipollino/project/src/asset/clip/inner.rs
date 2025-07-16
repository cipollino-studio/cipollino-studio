
use crate::{Color, LayerPtr, Objects, PaletteInner, Project};

use super::Clip;

/// The contents of a clip that are only loaded when the clip is opened by the user.
/// This is split into a separate object from Clip because we still need to load some basic 
/// information about the clip to render the assets panel(e.g the name of the clip).
#[derive(alisa::Serializable, Clone)]
pub struct ClipInner {
    pub layers: alisa::ChildList<LayerPtr>,
    pub colors: alisa::UnorderedChildList<alisa::OwningPtr<Color>>,

    pub width: u32,
    pub height: u32, 
    /// The length of the clip in frames
    pub length: u32,
    pub framerate: f32,
    pub background_color: [f32; 3],

    pub palettes: Vec<alisa::LoadingPtr<PaletteInner>>
}

impl Default for ClipInner {

    fn default() -> Self {
        Self {
            layers: Default::default(),
            colors: Default::default(),
            width: 1920,
            height: 1080,
            length: 100,
            framerate: 24.0,
            background_color: [1.0, 1.0, 1.0],
            palettes: Vec::new()
        }
    }

}

impl alisa::Object for ClipInner {
    type Project = Project;

    const TYPE_ID: u16 = 4;

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.clip_inners
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.clip_inners
    }
}

impl ClipInner {

    /// The length of a single frame, in seconds
    pub fn frame_len(&self) -> f32 {
        1.0 / self.framerate
    }

    /// The index of the frame at time t seconds
    pub fn frame_idx(&self, t: f32) -> i32 {
        ((t / self.frame_len()).floor() as i32).max(0)
    }

    /// The duration of the clip in seconds
    pub fn duration(&self) -> f32 { 
        (self.length as f32) * self.frame_len()
    }

}

#[derive(alisa::Serializable, Default)]
pub struct CreateClipInner {
    pub clip: alisa::Ptr<Clip>,
    pub inner: alisa::Ptr<ClipInner>
}

impl alisa::Operation for CreateClipInner {
    type Project = Project;
    const NAME: &'static str = "CreateClipInner";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Self::Project>) -> bool {

        let Some(clip) = recorder.get_obj(self.clip) else {
            return false;
        };
        let old_inner = clip.inner.ptr();
        if recorder.get_obj(old_inner).is_some() {
            return false;
        }

        recorder.add_obj(self.inner, ClipInner::default());
        let Some(clip) = recorder.get_obj_mut(self.clip) else {
            return false;
        };
        clip.inner = self.inner.into();

        true
    }

}

alisa::object_set_property_operation!(ClipInner, width, u32);
alisa::object_set_property_operation!(ClipInner, height, u32);
alisa::object_set_property_operation!(ClipInner, length, u32);
alisa::object_set_property_operation!(ClipInner, framerate, f32);
alisa::object_set_property_operation!(ClipInner, background_color, [f32; 3]);

#[derive(alisa::Serializable, Default)]
pub struct AddPaletteToClip {
    pub clip: alisa::Ptr<ClipInner>,
    pub palette: alisa::Ptr<PaletteInner>
}

#[derive(alisa::Serializable, Default)]
pub struct RemovePaletteFromClip {
    pub clip: alisa::Ptr<ClipInner>,
    pub palette: alisa::Ptr<PaletteInner>
}

impl alisa::Operation for AddPaletteToClip {
    type Project = Project;

    const NAME: &'static str = "AddPaletteToClip";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Self::Project>) -> bool {
        let Some(clip) = recorder.get_obj_mut(self.clip) else { return false; };
        for palette in clip.palettes.iter() {
            if palette.ptr() == self.palette {
                return true;
            }
        }
        clip.palettes.push(self.palette.into());
        true
    }
}

impl alisa::Operation for RemovePaletteFromClip {
    type Project = Project;
    const NAME: &'static str = "RemovePaletteFromClip";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Self::Project>) -> bool {
        let Some(clip) = recorder.get_obj_mut(self.clip) else { return false; };
        clip.palettes.retain(|palette| palette.ptr() != self.palette);
        true
    }
}

impl alisa::InvertibleOperation for AddPaletteToClip {
    type Inverse = RemovePaletteFromClip;

    fn inverse(&self, context: &alisa::ProjectContext<Self::Project>) -> Option<Self::Inverse> {
        let clip = context.obj_list().get(self.clip)?;
        for palette in clip.palettes.iter() {
            if palette.ptr() == self.palette {
                return None;
            }
        }
        Some(RemovePaletteFromClip {
            clip: self.clip,
            palette: self.palette
        })
    }
}

impl alisa::InvertibleOperation for RemovePaletteFromClip {
    type Inverse = AddPaletteToClip;

    fn inverse(&self, context: &alisa::ProjectContext<Self::Project>) -> Option<Self::Inverse> {
        context.obj_list().get(self.clip)?;
        Some(AddPaletteToClip {
            clip: self.clip,
            palette: self.palette
        })
    }
}
