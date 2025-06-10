use crate::{AudioClip, Objects, Project};


#[derive(Clone)]
pub struct AudioBlock {
    pub data: Box<[u8]>
}

impl alisa::Serializable for AudioBlock {

    fn serialize(&self, _context: &alisa::SerializationContext) -> alisa::ABFValue {
        alisa::ABFValue::Binary(self.data.clone())
    }

    fn deserialize(data: &alisa::ABFValue, _context: &mut alisa::DeserializationContext) -> Option<Self> {
        Some(Self {
            data: data.as_binary()?.into()
        })
    }

}

impl alisa::Object for AudioBlock {

    type Project = Project;
    const TYPE_ID: u16 = 13;

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.audio_blocks
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.audio_blocks
    }

}

#[derive(Default, alisa::Serializable)]
pub struct AddBlockToAudioClip {
    pub ptr: alisa::Ptr<AudioBlock>,
    pub clip: alisa::Ptr<AudioClip>,
    pub length: usize,
    pub data: Box<[u8]>
}

impl alisa::Operation for AddBlockToAudioClip {
    type Project = Project;

    const NAME: &'static str = "AddBlockToAudioClip";

    fn perform(&self, recorder: &mut alisa::Recorder<'_, Self::Project>) -> bool {
        recorder.add_obj(self.ptr, AudioBlock {
            data: self.data.clone()
        });

        let Some(clip) = recorder.get_obj_mut(self.clip) else { return false; };
        clip.blocks.push((self.length, self.ptr));
        clip.length += self.length;

        true
    }
}
