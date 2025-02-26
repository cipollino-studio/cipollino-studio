
use crate::{asset_operations, Action, Asset, Client, Folder, LayerChildList, LayerChildListTreeData, LayerParent, Objects, Project};

#[derive(alisa::Serializable, Clone)]
#[project(Project)]
pub struct Clip {
    pub folder: alisa::Ptr<Folder>,

    pub name: String,
    /// The length of the clip in frames
    pub length: u32,
    pub framerate: f32,

    pub layers: LayerChildList
}

impl Default for Clip {

    fn default() -> Self {
        Self {
            folder: alisa::Ptr::null(),
            name: "Clip".to_owned(),
            length: 100,
            framerate: 24.0,
            layers: LayerChildList::default()
        }
    }

}

impl alisa::Object for Clip {

    type Project = Project;

    const NAME: &'static str = "Clip";

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.clips
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.clips
    }
}

#[derive(alisa::Serializable)]
#[project(Project)]
pub struct ClipTreeData {
    pub name: String,
    pub length: u32,
    pub framerate: f32,
    pub layers: LayerChildListTreeData
}

impl Default for ClipTreeData {

    fn default() -> Self {
        Self {
            name: "Clip".to_owned(),
            length: 100,
            framerate: 24.0,
            layers: Default::default()
        }
    }

}

impl alisa::TreeObj for Clip {
    type ParentPtr = alisa::Ptr<Folder>;
    type ChildList = alisa::UnorderedChildList<Clip>;
    type TreeData = ClipTreeData;

    fn child_list<'a>(parent: alisa::Ptr<Folder>, context: &'a alisa::ProjectContext<Project>) -> Option<&'a Self::ChildList> {
        if parent.is_null() {
            return Some(&context.project().clips);
        }
        context.obj_list().get(parent).map(|folder| &folder.clips)
    }

    fn child_list_mut<'a>(parent: alisa::Ptr<Folder>, context: &'a mut alisa::ProjectContextMut<Project>) -> Option<&'a mut Self::ChildList> {
        if parent.is_null() {
            return Some(&mut context.project_mut().clips);
        }
        context.obj_list_mut().get_mut(parent).map(|folder| &mut folder.clips)
    }

    fn parent(&self) -> Self::ParentPtr {
        self.folder
    }

    fn parent_mut(&mut self) -> &mut Self::ParentPtr {
        &mut self.folder
    }

    fn instance(data: &Self::TreeData, ptr: alisa::Ptr<Self>, parent: alisa::Ptr<Folder>, recorder: &mut alisa::Recorder<Project>) {
        use alisa::Object;
        let clip = Self {
            folder: parent,
            name: data.name.clone(),
            length: data.length,
            framerate: data.framerate,
            layers: data.layers.instance(LayerParent::Clip(ptr), recorder)
        };
        Self::add(recorder, ptr, clip);
    }

    fn destroy(&self, recorder: &mut alisa::Recorder<Project>) {
        self.layers.destroy(recorder);
    }

    fn collect_data(&self, objects: &Objects) -> Self::TreeData {
        ClipTreeData {
            name: self.name.clone(),
            length: self.length,
            framerate: self.framerate,
            layers: self.layers.collect_data(objects)
        }
    }
}

impl Asset for Clip {

    fn name(&self) -> &String {
        &self.name
    }

    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    fn rename(client: &Client, action: &mut Action, ptr: alisa::Ptr<Self>, name: String) {
        client.perform(action, RenameClip {
            ptr,
            name,
        });
    }

    fn delete(client: &Client, action: &mut Action, ptr: alisa::Ptr<Self>) {
        client.perform(action, DeleteClip {
            ptr,
        });
    }

}

asset_operations!(Clip);

impl Clip {

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
