
use crate::{asset_operations, Action, Client, Objects, Project};

use super::{Asset, Folder};

mod block;
pub use block::*;

mod format;
pub use format::*;

#[derive(Clone, alisa::Serializable)]
pub struct AudioClip {
    pub folder: alisa::Ptr<Folder>,
    pub name: String,
    pub format: AudioFormat,
    /// Length in samples
    pub length: usize,
    pub blocks: Vec<(usize, alisa::Ptr<AudioBlock>)>
}

impl Default for AudioClip {

    fn default() -> Self {
        Self {
            folder: alisa::Ptr::null(),
            name: "Audio".to_owned(),
            format: AudioFormat::default(),
            length: 0,
            blocks: Vec::new()
        }
    }

}

impl alisa::Object for AudioClip {

    type Project = Project;
    const TYPE_ID: u16 = 12;

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.audio_clips
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.audio_clips
    }

}

#[derive(alisa::Serializable)]
pub struct AudioClipTreeData {
    pub name: String,
    pub format: AudioFormat,
    pub length: usize,
    pub blocks: Vec<(alisa::Ptr<AudioBlock>, usize, Box<[u8]>)>
}

impl Default for AudioClipTreeData {

    fn default() -> Self {
        Self {
            name: "Audio".to_owned(),
            format: Default::default(),
            length: 0,
            blocks: Vec::new()
        }
    }

}

impl alisa::TreeObj for AudioClip {
    type ParentPtr = alisa::Ptr<Folder>;
    type ChildList = alisa::UnorderedChildList<alisa::LoadingPtr<AudioClip>>;
    type TreeData = AudioClipTreeData;

    fn child_list<'a>(parent: Self::ParentPtr, context: &'a alisa::ProjectContext<Self::Project>) -> Option<&'a Self::ChildList> {
        if parent.is_null() {
            return Some(&context.project().audio_clips);
        }
        context.obj_list().get(parent).map(|folder| &folder.audio_clips)
    }

    fn child_list_mut<'a>(parent: Self::ParentPtr, recorder: &'a mut alisa::Recorder<Self::Project>) -> Option<&'a mut Self::ChildList> {
        if parent.is_null() {
            return Some(&mut recorder.project_mut().audio_clips);
        }
        recorder.get_obj_mut(parent).map(|folder| &mut folder.audio_clips)
    }

    fn parent(&self) -> Self::ParentPtr {
        self.folder
    }

    fn parent_mut(&mut self) -> &mut Self::ParentPtr {
        &mut self.folder
    }

    fn instance(data: &Self::TreeData, ptr: alisa::Ptr<Self>, parent: Self::ParentPtr, recorder: &mut alisa::Recorder<Self::Project>) {
        let mut blocks = Vec::new();
        for (ptr, size, block) in &data.blocks {
            recorder.add_obj(*ptr, AudioBlock {
                data: block.clone(),
            });
            blocks.push((*size, *ptr));
        } 
        recorder.add_obj(ptr, Self {
            folder: parent,
            format: data.format,
            length: data.length,
            name: data.name.clone(),
            blocks,
        });
    }

    fn destroy(&self, recorder: &mut alisa::Recorder<Self::Project>) {
        for (_, block) in &self.blocks {
            recorder.delete_obj(*block);
        }
    }

    fn collect_data(&self, objects: &Objects) -> Self::TreeData {
        let mut blocks = Vec::new();

        for (size, block_ptr) in &self.blocks {
            let Some(block) = objects.audio_blocks.get(*block_ptr) else { continue; }; 
            blocks.push((*block_ptr, *size, block.data.clone()));
        }

        AudioClipTreeData {
            name: self.name.clone(),
            format: self.format,
            length: self.length,
            blocks
        }
    }

    fn can_delete(ptr: alisa::Ptr<Self>, project: &alisa::ProjectContext<Self::Project>, source: alisa::OperationSource) -> bool {
        // If the server tells us to delete the clip, we should probably do that
        if source == alisa::OperationSource::Server {
            return true;
        }
        let Some(audio) = project.obj_list().get(ptr) else { return false; };

        for (_, block) in &audio.blocks {
            if !project.obj_list().get(*block).is_some() {
                return false;
            }
        }

        true
    }

}

impl Asset for AudioClip {

    const NAME: &'static str = "Audio Clip";

    fn name(&self) -> &String {
        &self.name
    }

    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    fn rename(action: &mut Action, ptr: alisa::Ptr<Self>, name: String) {
        action.push(RenameAudioClip {
            ptr,
            name,
        });
    }

    fn delete(action: &mut Action, ptr: alisa::Ptr<Self>) {
        action.push(DeleteAudioClip {
            ptr,
        });
    }

}

asset_operations!(AudioClip);

pub fn deep_load_audio_clip(audio_ptr: alisa::Ptr<AudioClip>, client: &Client) {
    let Some(audio) = client.get(audio_ptr) else {
        return;
    };
    
    for (_, block) in &audio.blocks {
        client.request_load(*block);
    }
}
