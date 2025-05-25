
use alisa::Ptr;
use project::{Action, Client, CreateFill, CreateStroke, FillPaths, FillTreeData, SceneObjPtr, SceneObjPtrTreeData, SceneObjectColor, StrokeData, StrokeTreeData};

use crate::{get_active_frame, EditorState, SceneRenderList, Selection};

pub struct StrokeClipboard {
    pub stroke: malvina::Stroke,
    pub color: SceneObjectColor,
    pub width: f32
}

impl StrokeClipboard {

    fn to_tree_data(&self) -> StrokeTreeData {
        StrokeTreeData {
            stroke: StrokeData(self.stroke.clone()),
            color: self.color,
            width: self.width,
        }
    } 

}

pub struct FillClipboard {
    pub paths: malvina::FillPaths,
    pub color: SceneObjectColor 
}

impl FillClipboard {
    
    fn to_tree_data(&self) -> FillTreeData {
        FillTreeData {
            paths: FillPaths(self.paths.clone()),
            color: self.color
        }
    }

}

pub enum SceneClipboardObject {
    Stroke(StrokeClipboard),
    Fill(FillClipboard)
}

impl SceneClipboardObject {

    pub(super) fn tree_data(&self, key: u64) -> (SceneObjPtr, SceneObjPtrTreeData) {
        match self {
            SceneClipboardObject::Stroke(stroke) => (
                SceneObjPtr::Stroke(Ptr::from_key(key)),
                SceneObjPtrTreeData::Stroke(Ptr::from_key(key), stroke.to_tree_data())
            ),
            SceneClipboardObject::Fill(fill) => (
                SceneObjPtr::Fill(Ptr::from_key(key)),
                SceneObjPtrTreeData::Fill(Ptr::from_key(key), fill.to_tree_data())
            ) 
        }
    }

}

pub struct SceneClipboard {
    pub objects: Vec<SceneClipboardObject>
}

impl SceneClipboard {

    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, obj: SceneObjPtr, client: &Client) {
        match obj {
            SceneObjPtr::Stroke(stroke_ptr) => {
                let Some(stroke) = client.get(stroke_ptr) else { return; };
                self.objects.push(SceneClipboardObject::Stroke(StrokeClipboard {
                    stroke: stroke.stroke.0.clone(),
                    color: stroke.color.into(),
                    width: stroke.width 
                }));
            },
            SceneObjPtr::Fill(fill_ptr) => {
                let Some(fill) = client.get(fill_ptr) else { return; };
                self.objects.push(SceneClipboardObject::Fill(FillClipboard {
                    paths: fill.paths.0.clone(),
                    color: fill.color.into(),
                }));
            }
        }
    }

}

impl Selection {
    
    pub(super) fn collect_scene_clipboard(&self, client: &Client, render_list: &SceneRenderList) -> SceneClipboard {
        let mut clipboard = SceneClipboard::new();
        for obj in render_list.objs.iter() {
            match obj {
                SceneObjPtr::Stroke(stroke) => if self.selected(*stroke) { clipboard.add_object(*obj, client); },
                SceneObjPtr::Fill(fill) => if self.selected(*fill) { clipboard.add_object(*obj, client); },
            }
        }
        clipboard
    }

}

impl SceneClipboard {

    pub fn paste(&self, client: &Client, editor: &EditorState) -> Option<Selection> {
        let mut action = Action::new(editor.action_context("Paste Strokes"));
        let frame = get_active_frame(client, editor, &mut action)?;
        let mut selection = Selection::new();
        for obj in &self.objects {
            match obj {
                SceneClipboardObject::Stroke(stroke) => {
                    let ptr = client.next_ptr();
                    action.push(CreateStroke {
                        ptr,
                        parent: frame,
                        idx: 0,
                        data: stroke.to_tree_data(),
                    });
                    selection.select(ptr);
                },
                SceneClipboardObject::Fill(fill) => {
                    let ptr = client.next_ptr();
                    action.push(CreateFill {
                        ptr,
                        parent: frame,
                        idx: 0,
                        data: fill.to_tree_data(),
                    });
                    selection.select(ptr);
                }
            } 
        }
        client.queue_action(action);
        Some(selection)
    }

}
