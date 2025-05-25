
use crate::{asset_operations, Action, Client, Color, ColorParent, Objects, Project};

use super::{Asset, Folder};

mod inner;
pub use inner::*;

#[derive(alisa::Serializable, Clone)]
pub struct Palette {
    pub folder: alisa::Ptr<Folder>,
    pub name: String,
    pub inner: alisa::Ptr<PaletteInner>
}

impl Default for Palette {

    fn default() -> Self {
        Self {
            folder: alisa::Ptr::null(),
            name: "Palette".into(),
            inner: alisa::Ptr::null()
        }
    }

}

impl alisa::Object for Palette {

    type Project = Project;

    const TYPE_ID: u16 = 8;

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.palettes
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.palettes
    }

}

#[derive(alisa::Serializable)]
pub struct PaletteTreeData {
    pub name: String,

    pub inner_ptr: alisa::Ptr<PaletteInner>,
    pub colors: alisa::UnorderedChildListTreeData<alisa::LoadingPtr<Color>>
}

impl Default for PaletteTreeData {

    fn default() -> Self {
        Self {
            name: "Palette".to_owned(),
            inner_ptr: alisa::Ptr::null(),
            colors: Default::default()
        }
    }

}

impl alisa::TreeObj for Palette {
    type ParentPtr = alisa::Ptr<Folder>;
    type ChildList = alisa::UnorderedChildList<alisa::LoadingPtr<Palette>>;
    type TreeData = PaletteTreeData;

    fn child_list<'a>(parent: Self::ParentPtr, context: &'a alisa::ProjectContext<Self::Project>) -> Option<&'a Self::ChildList> {
        if parent.is_null() {
            return Some(&context.project().palettes);
        }
        context.obj_list().get(parent).map(|folder| &folder.palettes)
    }

    fn child_list_mut<'a>(parent: Self::ParentPtr, recorder: &'a mut alisa::Recorder<Self::Project>) -> Option<&'a mut Self::ChildList> {
        if parent.is_null() {
            return Some(&mut recorder.project_mut().palettes);
        }
        recorder.get_obj_mut(parent).map(|folder| &mut folder.palettes)
    }

    fn parent(&self) -> Self::ParentPtr {
        self.folder
    }

    fn parent_mut(&mut self) -> &mut Self::ParentPtr {
        &mut self.folder
    }

    fn instance(data: &Self::TreeData, ptr: alisa::Ptr<Self>, parent: Self::ParentPtr, recorder: &mut alisa::Recorder<Self::Project>) {
        let palette_inner = PaletteInner {
            colors: data.colors.instance(ColorParent::Palette(ptr), recorder)
        };
        recorder.add_obj(data.inner_ptr, palette_inner);

        let palette = Self {
            folder: parent,
            name: data.name.clone(),
            inner: data.inner_ptr,
        };
        recorder.add_obj(ptr, palette);
    }

    fn destroy(&self, recorder: &mut alisa::Recorder<Self::Project>) {
        if let Some(palette_inner) = recorder.delete_obj(self.inner) {
            palette_inner.colors.destroy(recorder); 
        }
    }

    fn collect_data(&self, objects: &<Self::Project as alisa::Project>::Objects) -> Self::TreeData {
        let palette_inner = objects.palette_inners.get(self.inner);
        let colors = palette_inner
            .map(|palette_inner| palette_inner.colors.collect_data(objects))
            .unwrap_or_default();

        PaletteTreeData {
            name: self.name.clone(),
            inner_ptr: self.inner,
            colors
        }
    }

    fn can_delete(ptr: alisa::Ptr<Self>, project: &alisa::ProjectContext<Project>, source: alisa::OperationSource) -> bool {
        // If the server tells us to delete the clip, we should probably do that
        if source == alisa::OperationSource::Server {
            return true;
        }
        let Some(palette) = project.obj_list().get(ptr) else { return false; };
        let inner_loaded = project.obj_list().get(palette.inner).is_some();
        inner_loaded
    }
}

impl Asset for Palette {
    const NAME: &'static str = "Palette";

    fn name(&self) -> &String {
        &self.name
    }

    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    fn rename(action: &mut Action, ptr: alisa::Ptr<Self>, name: String) {
        action.push(RenamePalette {
            ptr,
            name,
        });
    }

    fn delete(action: &mut Action, ptr: alisa::Ptr<Self>) {
        action.push(DeletePalette {
            ptr
        });
    }
}

asset_operations!(Palette);

pub fn deep_load_palette(palette_ptr: alisa::Ptr<Palette>, client: &Client) {
    let Some(palette) = client.get(palette_ptr) else {
        return;
    };
    
    if client.get_ref(palette.inner).is_deleted() {
        client.queue_operation(CreatePaletteInner {
            palette: palette_ptr, 
            inner: client.next_ptr(),
        });
    } else {
        client.request_load(palette.inner);
    }
}
