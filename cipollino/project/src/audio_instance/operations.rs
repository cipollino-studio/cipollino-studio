
use alisa::TreeObj;

use crate::{AudioClip, AudioLayer, Project};

use super::{AudioInstance, AudioInstanceTreeData};

#[derive(Default, alisa::Serializable)]
pub struct CreateAudioInstance {
    pub ptr: alisa::Ptr<AudioInstance>,
    pub layer: alisa::Ptr<AudioLayer>,
    pub clip: alisa::Ptr<AudioClip>,
    pub start: f32,
    pub end: f32
}

impl alisa::Operation for CreateAudioInstance {
    type Project = Project;
    const NAME: &'static str = "CreateAudioInstance";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Self::Project>) -> bool {
        if self.end <= self.start {
            return false;
        }

        let Some(layer) = recorder.get_obj(self.layer) else { return false; };

        for audio in layer.audio_instances.iter() {
            let Some(audio) = recorder.get_obj(audio.ptr()) else { continue; };
            if elic::Range::new(self.start, self.end).intersects(elic::Range::new(audio.start, audio.end)) {
                return false;
            }
        }

        alisa::create_tree_object(recorder, self.ptr, self.layer, (), &AudioInstanceTreeData {
            clip: self.clip,
            start: self.start,
            end: self.end,
        })
    }
}

impl alisa::InvertibleOperation for CreateAudioInstance {
    type Inverse = DeleteAudioInstance;

    fn inverse(&self, _context: &alisa::ProjectContext<Self::Project>) -> Option<Self::Inverse> {
        Some(DeleteAudioInstance {
            ptr: self.ptr,
        })
    }
}

#[derive(Default, alisa::Serializable)]
pub struct DeleteAudioInstance {
    pub ptr: alisa::Ptr<AudioInstance>
}

impl alisa::Operation for DeleteAudioInstance {
    type Project = Project;
    const NAME: &'static str = "DeleteAudioInstance";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Self::Project>) -> bool {
        if !AudioInstance::can_delete(self.ptr, &recorder.context(), recorder.source()) {
            return false;
        }
        alisa::delete_tree_object(recorder, self.ptr)
    }
}

impl alisa::InvertibleOperation for DeleteAudioInstance {
    type Inverse = CreateAudioInstance;

    fn inverse(&self, context: &alisa::ProjectContext<Self::Project>) -> Option<Self::Inverse> {
        let audio = context.obj_list().get(self.ptr)?;
        Some(CreateAudioInstance {
            ptr: self.ptr,
            layer: audio.layer,
            clip: audio.clip,
            start: audio.start,
            end: audio.end
        })
    }
}
