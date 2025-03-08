
pub mod layout;
use std::{collections::HashMap, hash::Hash};

pub use layout::*;

pub mod memory;
pub use memory::*;

mod style;
pub use style::*;

pub mod input;
pub use input::*;

mod paint;

mod node_params;
pub use node_params::*;

mod tree;
pub use tree::*;

mod cursor;
pub use cursor::*;

mod texture;

mod clipboard;

mod redraw_signal;
pub use redraw_signal::*;

use crate::{Color, Rect, Vec2};

use super::{hash, text::FontId, Margin, Painter, PerAxis, RenderResources, Stroke, TSTransform, Texture};

pub struct UI<'a, 'b> {
    input: &'a Input,
    memory: &'a mut Memory,
    style: StyleStack,

    render_resources: &'a mut RenderResources<'b>,
    clipboard: Option<&'a mut arboard::Clipboard>,

    textures: &'a mut HashMap<String, Texture>,

    window_size: Vec2,

    // tree-building
    tree: UITree,
    parent_stack: Vec<UIRef>,
    curr_sibling: UIRef,

    // communication
    pub(crate) request_redraw: bool,
    pub(crate) cursor: CursorIcon,
    pub(crate) request_ime: Option<UIRef>
}

impl<'a, 'b> UI<'a, 'b> {

    pub(crate) fn new(input: &'a Input, memory: &'a mut Memory, render_resources: &'a mut RenderResources<'b>, clipboard: Option<&'a mut arboard::Clipboard>, textures: &'a mut HashMap<String, Texture>, window_size: Vec2, tree: UITree, layer: UIRef) -> Self {
        Self {
            input,
            memory,
            style: StyleStack::new(),
            render_resources,
            textures,
            clipboard,
            window_size,
            tree,
            parent_stack: vec![layer],
            curr_sibling: UIRef::Null,
            request_redraw: false,
            cursor: CursorIcon::default(),
            request_ime: None
        }
    }

    pub(crate) fn tree(self) -> UITree {
        self.tree
    }

    pub fn curr_parent(&self) -> UIRef {
        let Some(parent) = self.parent_stack.last() else { panic!("no parents in parent stack. ui in an invalid state.") };
        *parent
    }

    pub fn node(&mut self, params: UINodeParams) -> Response {
        let parent_ref = self.curr_parent();
        let parent = self.tree.get_mut(parent_ref);

        let mut new_node = UINode::new(parent.id, parent.curr_id_seed, params);
        parent.curr_id_seed += 1;

        new_node.prev = self.curr_sibling;
        new_node.parent = parent_ref;

        let new_node = self.tree.add_node(new_node);

        match self.curr_sibling {
            UIRef::Null => {
                self.tree.get_mut(parent_ref).first_child = new_node;
            },
            UIRef::Some(_) => {
                self.tree.get_mut(self.curr_sibling).next = new_node;
            },
        }

        self.curr_sibling = new_node;
        self.tree.get_mut(parent_ref).last_child = new_node;
        self.tree.get_mut(parent_ref).n_children += 1; 

        let id = self.tree.get(new_node).id;
        let interaction = self.memory.get::<Interaction>(id);
        
        Response {
            id,
            node_ref: new_node,
            hovered: interaction.hovered,
            l_mouse: interaction.l_mouse,
            r_mouse: interaction.r_mouse,
            scroll: interaction.scroll,
            dnd_hovered: interaction.dnd_hovered
        }
    }

    pub(crate) fn push_parent(&mut self, parent: UIRef) {
        self.parent_stack.push(parent);
        self.curr_sibling = self.tree.get(parent).last_child;
    }

    pub(crate) fn pop_parent(&mut self) {
        self.parent_stack.pop();
        if self.parent_stack.is_empty() {
            panic!("ui parent stack underflow!");
        }
        self.curr_sibling = self.tree.get(self.curr_parent()).last_child;
    }

    pub fn with_parent<R, F: FnOnce(&mut Self) -> R>(&mut self, parent: UIRef, body: F) -> R {
        self.push_parent(parent);
        let body_result = body(self);
        self.pop_parent();
        body_result
    }

    pub fn with_node<R, F: FnOnce(&mut UI) -> R>(&mut self, params: UINodeParams, body: F) -> (Response, R) {
        let resp = self.node(params);
        (resp, self.with_parent(resp.node_ref, body))
    }

    pub fn layer<R, F: FnOnce(&mut Self) -> R>(&mut self, body: F) -> (UIRef, R) {
        let layer = self.tree.add_layer(self.window_size);
        (layer, self.with_parent(layer, body))
    }

    pub fn get_parent_ref(&self, node: UIRef) -> UIRef {
        self.tree.get(node).parent
    } 

    pub fn get_parent_id(&self, node: UIRef) -> Id {
        let parent_ref = self.get_parent_ref(node);
        self.tree.get(parent_ref).id
    }

    pub fn get_node_id(&self, node: UIRef) -> Id {
        self.tree.get(node).id
    }

    pub fn push_id_seed<H: Hash>(&mut self, source: &H) {
        let parent_ref = self.curr_parent();
        let parent = self.tree.get_mut(parent_ref);
        parent.curr_id_seed = hash(source);
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn window_size(&self) -> Vec2 {
        self.window_size
    }

    pub fn memory(&mut self) -> &mut Memory {
        &mut self.memory
    }

    pub fn style<S: Style>(&mut self) -> S::Value {
        self.style.get::<S>()
    }

    pub fn push_style<S: Style>(&mut self, style: S::Value) {
        self.style.push::<S>(style);
    }

    pub fn pop_style(&mut self) {
        self.style.pop();
    }

    pub fn with_style<S: Style, R, F: FnOnce(&mut Self) -> R>(&mut self, style: S::Value, body: F) -> R {
        self.style.push::<S>(style);
        let result = body(self);
        self.style.pop();
        result
    }

    pub fn set_size(&mut self, node: UIRef, width: Size, height: Size) {
        self.tree.get_mut(node).params.size = PerAxis::new(width, height);
    }
    
    pub fn set_margin(&mut self, node: UIRef, margin: Margin) {
        self.tree.get_mut(node).params.margin = margin; 
    }

    pub fn set_fill(&mut self, node: UIRef, fill: Color) {
        self.tree.get_mut(node).params.fill = fill;
    }

    pub fn set_texture(&mut self, node: UIRef, texture: Texture) {
        self.tree.get_mut(node).params.texture = Some(texture); 
    }
    
    pub fn set_text_color(&mut self, node: UIRef, color: Color) {
        self.tree.get_mut(node).params.text_style.color = color;
    }

    pub fn set_stroke(&mut self, node: UIRef, stroke: Stroke) {
        self.tree.get_mut(node).params.stroke = stroke;
    }

    pub fn set_transform(&mut self, node: UIRef, transform: TSTransform) {
        self.tree.get_mut(node).params.transform = transform;
    }

    pub fn set_text<S: Into<String>>(&mut self, node: UIRef, text: S) {
        self.tree.get_mut(node).params.text = Some(text.into());
    }

    pub fn set_sense_mouse(&mut self, node: UIRef, mouse: bool) {
        self.tree.get_mut(node).params.sense.set(Sense::MOUSE, mouse);
    }

    pub fn set_sense_scroll(&mut self, node: UIRef, scroll: bool) {
        self.tree.get_mut(node).params.sense.set(Sense::SCROLL, scroll);
    }

    pub fn set_sense_dnd_hover(&mut self, node: UIRef, dnd_hover: bool) {
        self.tree.get_mut(node).params.sense.set(Sense::DND_HOVER, dnd_hover);
    }
    
    pub fn set_on_paint<F: FnOnce(&mut Painter, Rect) + 'static>(&mut self, node: UIRef, on_paint: F) {
        self.tree.get_mut(node).params.on_paint = Some(Box::new(on_paint));
    }

    pub fn request_redraw(&mut self) {
        self.request_redraw = true;
    }

    pub fn set_cursor(&mut self, cursor: CursorIcon) {
        self.cursor = cursor;
    }

    pub fn request_ime(&mut self, node: UIRef) {
        self.request_ime = Some(node);
    }

    /// Get the WebGPU render device
    pub fn wgpu_device(&self) -> &wgpu::Device {
        &self.render_resources.device
    } 

    /// Get the WebGPU render queue
    pub fn wgpu_queue(&self) -> &wgpu::Queue {
        &self.render_resources.queue
    }

    /// Get the COSMIC Text font system
    pub fn font_system(&mut self, font_id: FontId) -> Option<&mut cosmic_text::FontSystem> {
        let font = self.render_resources.text_resources.fonts.get_mut(&font_id)?;
        Some(&mut font.font_system)
    }

    pub fn text_font(&self) -> FontId {
        self.render_resources.text_resources.text_font
    }
    
    pub fn icon_font(&self) -> FontId {
        self.render_resources.text_resources.icon_font
    }

    pub fn scale_factor(&self) -> f32 {
        self.render_resources.window.scale_factor() as f32
    }

}
