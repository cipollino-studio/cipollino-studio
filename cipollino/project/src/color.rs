
use crate::{Clip, Objects, Palette, Project};

alisa::ptr_enum!(ColorParent [Clip, Palette]);

impl Default for ColorParent {

    fn default() -> Self {
        ColorParent::Clip(alisa::Ptr::null())
    }

}

#[derive(alisa::Serializable, Clone)]
pub struct Color {
    pub parent: ColorParent,
    pub color: [f32; 3],
    pub name: String
}

impl Default for Color {

    fn default() -> Self {
        Self {
            parent: Default::default(),
            color: [0.0; 3],
            name: "Color".to_owned()
        }
    }
    
}

impl alisa::Object for Color {
    type Project = Project;

    const TYPE_ID: u16 = 10;

    fn list(objects: &Objects) -> &alisa::ObjList<Self> {
        &objects.colors
    }

    fn list_mut(objects: &mut Objects) -> &mut alisa::ObjList<Self> {
        &mut objects.colors
    }
}

#[derive(alisa::Serializable)]
pub struct ColorTreeData {
    pub color: [f32; 3],
    pub name: String
}

impl Default for ColorTreeData {

    fn default() -> Self {
        Self {
            color: [0.0; 3],
            name: "Color".to_owned()
        }
    }

}

impl alisa::TreeObj for Color {
    type ParentPtr = ColorParent;
    type ChildList = alisa::UnorderedChildList<alisa::LoadingPtr<Color>>;
    type TreeData = ColorTreeData;

    fn child_list<'a>(parent: Self::ParentPtr, context: &'a alisa::ProjectContext<Self::Project>) -> Option<&'a Self::ChildList> {
        match parent {
            ColorParent::Clip(ptr) => {
                let clip_inner = context.obj_list().get(ptr)?.inner;
                Some(&context.obj_list().get(clip_inner)?.colors)
            },
            ColorParent::Palette(ptr) => {
                let palette_inner = context.obj_list().get(ptr)?.inner;
                Some(&context.obj_list().get(palette_inner)?.colors)
            }
        }
    }

    fn child_list_mut<'a>(parent: Self::ParentPtr, recorder: &'a mut alisa::Recorder<Self::Project>) -> Option<&'a mut Self::ChildList> {
        match parent {
            ColorParent::Clip(ptr) => {
                let clip_inner = recorder.get_obj_mut(ptr)?.inner;
                Some(&mut recorder.get_obj_mut(clip_inner)?.colors)
            },
            ColorParent::Palette(ptr) => {
                let palette_inner = recorder.get_obj_mut(ptr)?.inner;
                Some(&mut recorder.get_obj_mut(palette_inner)?.colors)
            }
        }
    }

    fn parent(&self) -> Self::ParentPtr {
        self.parent
    }

    fn parent_mut(&mut self) -> &mut Self::ParentPtr {
        &mut self.parent
    }

    fn instance(data: &Self::TreeData, ptr: alisa::Ptr<Self>, parent: Self::ParentPtr, recorder: &mut alisa::Recorder<Self::Project>) {
        recorder.add_obj(ptr, Color {
            parent: parent,
            color: data.color,
            name: data.name.clone()
        });
    }

    fn destroy(&self, _recorder: &mut alisa::Recorder<Self::Project>) {

    }

    fn collect_data(&self, _objects: &<Self::Project as alisa::Project>::Objects) -> Self::TreeData {
        ColorTreeData {
            color: self.color,
            name: self.name.clone()
        }
    }

}

alisa::tree_object_creation_operations!(Color);
alisa::object_set_property_operation!(Color, color, [f32; 3]);
alisa::object_set_property_operation!(Color, name, String);

#[derive(Clone, Copy)]
pub struct SceneObjectColor {
    pub color: alisa::LoadingPtr<Color>,
    pub backup: [f32; 3],
}

impl Default for SceneObjectColor {

    fn default() -> Self {
        Self {
            color: Default::default(),
            backup: [0.0; 3] 
        }
    }

}

impl alisa::Serializable for SceneObjectColor {

    fn serialize(&self, context: &alisa::SerializationContext) -> alisa::ABFValue {
        alisa::ABFValue::Map(Box::new([
            ("color".into(), self.color.serialize(context)),
            ("backup".into(), alisa::ABFValue::U32(u32::from_le_bytes([
                ((self.backup[0] * 255.0).round() as i32).clamp(0, 255) as u8,
                ((self.backup[1] * 255.0).round() as i32).clamp(0, 255) as u8,
                ((self.backup[2] * 255.0).round() as i32).clamp(0, 255) as u8,
                0
            ]))),
        ]))
    }

    fn deserialize(data: &alisa::ABFValue, context: &mut alisa::DeserializationContext) -> Option<Self> {
        match data {
            alisa::ABFValue::Array(arr) => {
                let r = arr.get(0)?.as_f32()?;
                let g = arr.get(1)?.as_f32()?;
                let b = arr.get(2)?.as_f32()?;
                return Some(SceneObjectColor {
                    color: alisa::LoadingPtr::new(alisa::Ptr::null()),
                    backup: [
                        r,
                        g,
                        b
                    ] 
                });
            },
            _ => {}
        } 
        let color = alisa::LoadingPtr::deserialize(data.get("color")?, context).unwrap_or_default();
        let backup = data.get("backup")?.as_u32().unwrap_or_default();
        let [r, g, b, _] = backup.to_le_bytes();
        Some(SceneObjectColor {
            color,
            backup: [
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0
            ]
        })
    }

}