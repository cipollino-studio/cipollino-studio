
use crate::{asset_operations, Action, Asset, Client, Folder, LayerParent, LayerPtr, Objects, Project};

mod inner;
pub use inner::*;

#[derive(alisa::Serializable, Clone)]
pub struct Clip {
    pub folder: alisa::Ptr<Folder>,

    pub name: String,

    pub inner: alisa::Ptr<ClipInner>
}

impl Default for Clip {

    fn default() -> Self {
        Self {
            folder: alisa::Ptr::null(),
            name: "Clip".to_owned(),
            inner: alisa::Ptr::null()
        }
    }

}

impl alisa::Object for Clip {

    type Project = Project;

    const TYPE_ID: u16 = 3;

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.clips
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.clips
    }
}

#[derive(alisa::Serializable)]
pub struct ClipTreeData {
    pub name: String,
    pub length: u32,
    pub framerate: f32,
    pub width: u32,
    pub height: u32,
    
    pub inner_ptr: alisa::Ptr<ClipInner>,
    pub layers: alisa::ChildListTreeData<LayerPtr> 
}

impl Default for ClipTreeData {

    fn default() -> Self {
        Self {
            name: "Clip".to_owned(),
            length: 100,
            framerate: 24.0,
            width: 1920,
            height: 1080,
            inner_ptr: alisa::Ptr::null(),
            layers: Default::default()
        }
    }

}

impl alisa::TreeObj for Clip {
    type ParentPtr = alisa::Ptr<Folder>;
    type ChildList = alisa::UnorderedChildList<alisa::LoadingPtr<Clip>>;
    type TreeData = ClipTreeData;

    fn child_list<'a>(parent: alisa::Ptr<Folder>, context: &'a alisa::ProjectContext<Project>) -> Option<&'a Self::ChildList> {
        if parent.is_null() {
            return Some(&context.project().clips);
        }
        context.obj_list().get(parent).map(|folder| &folder.clips)
    }

    fn child_list_mut<'a>(parent: alisa::Ptr<Folder>, context: &'a mut alisa::Recorder<Project>) -> Option<&'a mut Self::ChildList> {
        if parent.is_null() {
            return Some(&mut context.project_mut().clips);
        }
        context.get_obj_mut(parent).map(|folder| &mut folder.clips)
    }

    fn parent(&self) -> Self::ParentPtr {
        self.folder
    }

    fn parent_mut(&mut self) -> &mut Self::ParentPtr {
        &mut self.folder
    }

    fn instance(data: &Self::TreeData, ptr: alisa::Ptr<Self>, parent: alisa::Ptr<Folder>, recorder: &mut alisa::Recorder<Project>) {
        let clip_inner = ClipInner {
            layers: data.layers.instance(LayerParent::Clip(ptr), recorder),
            length: data.length,
            framerate: data.framerate,
            width: data.width,
            height: data.height,
        };
        recorder.add_obj(data.inner_ptr, clip_inner);

        let clip = Self {
            folder: parent,
            name: data.name.clone(),
            inner: data.inner_ptr 
        };
        recorder.add_obj(ptr, clip);
    }

    fn destroy(&self, recorder: &mut alisa::Recorder<Project>) {
        if let Some(clip_inner) = recorder.delete_obj(self.inner) {
            clip_inner.layers.clone().destroy(recorder);
        }
    }

    fn collect_data(&self, objects: &Objects) -> Self::TreeData {
        let clip_inner = objects.clip_inners.get(self.inner);
        let length = clip_inner.map(|inner| inner.length).unwrap_or(100);
        let framerate = clip_inner.map(|inner| inner.framerate).unwrap_or(24.0);
        let width = clip_inner.map(|inner| inner.width).unwrap_or(1920);
        let height = clip_inner.map(|inner| inner.width).unwrap_or(1080);
        let layers = clip_inner
            .map(|clip_inner| clip_inner.layers.collect_data(objects))
            .unwrap_or_default();

        ClipTreeData {
            name: self.name.clone(),
            length,
            framerate,
            inner_ptr: self.inner,
            layers,
            width,
            height, 
        }
    }

    fn can_delete(ptr: alisa::Ptr<Self>, project: &alisa::ProjectContext<Project>, source: alisa::OperationSource) -> bool {
        // If the server tells us to delete the clip, we should probably do that
        if source == alisa::OperationSource::Server {
            return true;
        }
        let Some(clip) = project.obj_list().get(ptr) else { return false; };
        let inner_loaded = project.obj_list().get(clip.inner).is_some();
        inner_loaded
    }

}

impl Asset for Clip {

    const NAME: &'static str = "Clip";

    fn name(&self) -> &String {
        &self.name
    }

    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    fn rename(action: &mut Action, ptr: alisa::Ptr<Self>, name: String) {
        action.push(RenameClip {
            ptr,
            name,
        });
    }

    fn delete(action: &mut Action, ptr: alisa::Ptr<Self>) {
        action.push(DeleteClip {
            ptr,
        });
    }

}

asset_operations!(Clip);

pub fn deep_load_clip(clip_ptr: alisa::Ptr<Clip>, client: &Client) {
    let Some(clip) = client.get(clip_ptr) else {
        return;
    };
    
    if client.get_ref(clip.inner).is_deleted() {
        client.queue_operation(CreateClipInner {
            clip: clip_ptr, 
            inner: client.next_ptr(),
        });
    } else {
        client.request_load(clip.inner);
    }
}
