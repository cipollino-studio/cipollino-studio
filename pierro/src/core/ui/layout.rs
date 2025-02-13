
use crate::{text::{FontId, TextResources}, vec2, Axis, PerAxis, Range, Rect, TSTransform, Vec2, AXES};

use super::{Id, Memory, UIRef, UITree};

#[derive(Clone, Copy)]
pub enum SizeKind {
    /// A constant number of pixels
    Px(f32),
    /// Big enough to fit the text content of the node
    Text,
    /// Some number of fractional units of the parent node's size.
    Fr(f32),
    /// Big enough to fit the children of the node
    Fit,
}

#[derive(Clone, Copy)]
pub struct Size {
    size: SizeKind,
    shrink: bool,
    grow: f32
}

impl Size {

    pub fn new(size: SizeKind) -> Self {
        Self {
            size,
            shrink: true,
            grow: 0.0
        }
    }

    pub fn px(size: f32) -> Self {
        Self::new(SizeKind::Px(size))
    }

    pub fn text() -> Self {
        Self::new(SizeKind::Text)
    }

    pub fn fr(frac: f32) -> Self {
        Self::new(SizeKind::Fr(frac))
    }
    
    pub fn fit() -> Self {
        Self::new(SizeKind::Fit)
    }

    pub fn no_shrink(mut self) -> Self {
        self.shrink = false;
        self
    }

    pub fn with_grow(mut self, grow: f32) -> Self {
        self.grow = grow;
        self
    }

}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Justify {
    Min,
    Center,
    Max
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Min,
    Center,
    Max
}

#[derive(Clone, Copy)]
pub struct Layout {
    axis: Axis,
    justify: Justify,
    align: Align,
    allow_overflow: PerAxis<bool>
}

impl Layout {

    pub fn new(axis: Axis) -> Self {
        Self {
            axis,
            justify: Justify::Min,
            align: Align::Min,
            allow_overflow: PerAxis::splat(false)
        }
    }

    pub fn horizontal() -> Self {
        Self::new(Axis::X)
    }

    pub fn vertical() -> Self {
        Self::new(Axis::Y)
    }

    pub fn with_justify(mut self, justify: Justify) -> Self {
        self.justify = justify;
        self
    }

    pub fn justify_min(self) -> Self {
        self.with_justify(Justify::Min)
    }

    pub fn justify_center(self) -> Self {
        self.with_justify(Justify::Center)
    }

    pub fn justify_max(self) -> Self {
        self.with_justify(Justify::Max)
    }

    pub fn with_align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn align_min(self) -> Self {
        self.with_align(Align::Min)
    }

    pub fn align_center(self) -> Self {
        self.with_align(Align::Center)
    }

    pub fn align_max(self) -> Self {
        self.with_align(Align::Max)
    }

    pub fn with_horizontal_overflow(mut self) -> Self {
        self.allow_overflow.x = true;
        self
    }

    pub fn with_vertical_overflow(mut self) -> Self {
        self.allow_overflow.y = true;
        self
    }

}

const TINY: f32 = 0.000000000000001;

impl UITree {

    fn count_child_fractional_units(&self, node: UIRef, axis: Axis) -> f32 {
        let on_main_axis = self.get(node).params.layout.axis == axis;

        let mut frac_units = 0.0;
        let mut child_ref = self.get(node).first_child;
        while child_ref.is_some() {
            let child = self.get(child_ref);
            if let SizeKind::Fr(frac) = child.params.size.on_axis(axis).size {
                if on_main_axis {
                    frac_units += frac;
                } else {
                    frac_units = frac_units.max(frac);
                }
            }
            child_ref = child.next; 
        }

        if matches!(self.get(node).params.size.on_axis(axis).size, SizeKind::Fit) {
            if on_main_axis {
                frac_units += 1.0;
            } else {
                frac_units = frac_units.max(1.0);
            }
        }

        frac_units
    }

    fn calc_text_size(&mut self, memory: &mut Memory, node: UIRef, axis: Axis, text_resources: &mut TextResources) -> f32 {
        let Some(text) = self.get(node).params.text.as_ref() else { return 0.0; };
        let text_style = self.get(node).params.text_style;

        let text_size_cache = memory.get::<TextSizeCache>(self.get(node).id); 
        if &text_size_cache.text == text && text_size_cache.font_size == text_style.font_size && text_size_cache.line_height == text_style.line_height && text_size_cache.font == text_style.font {
            return text_size_cache.size.on_axis(axis);
        }

        let Some(font) = text_resources.fonts.get_mut(&text_style.font) else { return 0.0; };
        let mut buffer = cosmic_text::Buffer::new(&mut font.font_system, cosmic_text::Metrics { font_size: text_style.font_size, line_height: text_style.font_size * text_style.line_height });
        buffer.set_text(&mut font.font_system, text, cosmic_text::Attrs::new().family(cosmic_text::Family::SansSerif), cosmic_text::Shaping::Advanced);
        let mut w: f32 = 0.0;
        let mut h: f32 = 0.0;
        for run in buffer.layout_runs() {
            w = w.max(run.line_w);
            h = h.max(run.line_height);
        }

        // The size, with a small margin of error for floating point rounding issues
        let size = vec2(w, h) + Vec2::splat(0.1);

        text_size_cache.text = text.clone();
        text_size_cache.font_size = text_style.font_size;
        text_size_cache.line_height = text_style.line_height;
        text_size_cache.font = text_style.font;
        text_size_cache.size = size;
        
        size.on_axis(axis)
    }

    fn calc_content_basis_size(&mut self, node: UIRef, axis: Axis) -> f32 {
        let layout_axis = self.get(node).params.layout.axis; 

        if axis == layout_axis {
            let mut content_size = 0.0;

            let mut child_ref = self.get(node).first_child;
            while child_ref.is_some() {
                let child = self.get(child_ref);
                content_size += child.basis_size.on_axis(axis);
                child_ref = self.get(child_ref).next;
            }

            content_size
        } else {
            let mut content_size: f32 = 0.0;

            let mut child_ref = self.get(node).first_child;
            while child_ref.is_some() {
                let child = self.get(child_ref);
                content_size = content_size.max(child.basis_size.on_axis(axis));
                child_ref = child.next;
            }

            content_size
        }
    }

    fn calc_down_dependent_basis_size(&mut self, memory: &mut Memory, node: UIRef, axis: Axis, text_resources: &mut TextResources) {
        let mut child = self.get(node).first_child;
        while child.is_some() {
            self.calc_down_dependent_basis_size(memory, child, axis, text_resources);
            child = self.get(child).next;
        }

        let frac_units = self.count_child_fractional_units(node, axis);
        let size = self.get(node).params.size.on_axis(axis).size;
        let basis_size = match size {
            SizeKind::Px(size) => size,
            SizeKind::Text => self.calc_text_size(memory, node, axis, text_resources),
            SizeKind::Fr(_) | SizeKind::Fit => self.calc_content_basis_size(node, axis),
        };

        let margin = self.get(node).params.margin.total().on_axis(axis);
        *self.get_mut(node).basis_size.on_axis_mut(axis) = basis_size + margin;
        *self.get_mut(node).frac_units.on_axis_mut(axis) = frac_units;
    }

    fn calc_up_dependent_basis_size(&mut self, node: UIRef, axis: Axis) {
    
        // Subtract the margins from the node's basis size.
        // This is done because the margins should not be factored
        // in to the size of the fractional unit for this node.
        let margin = self.get(node).params.margin.total().on_axis(axis);
        *self.get_mut(node).basis_size.on_axis_mut(axis) -= margin;

        // If the node is using a fractional size, set the node's basis size to that fraction of the parent
        let parent = self.get(node).parent;
        if parent.is_some() {
            if let SizeKind::Fr(frac) = self.get(node).params.size.on_axis(axis).size {
                let parent = self.get(parent);
                let parent_basis = parent.basis_size.on_axis(axis);
                let parent_frac_units = parent.frac_units.on_axis(axis);
                *self.get_mut(node).basis_size.on_axis_mut(axis) = parent_basis * frac / parent_frac_units;
            }
        }

        // Calculate the space taken up by the node's non-fractional children.
        // This is necessary to calculate how much space must be given to fractional children
        let mut non_frac_size = 0.0;
        let mut child_ref = self.get(node).first_child;
        while child_ref.is_some() {
            let child = self.get(child_ref);
            if !matches!(child.params.size.on_axis(axis).size, SizeKind::Fr(_)) {
                non_frac_size += child.basis_size.on_axis(axis);
            }
            child_ref = child.next;
        }

        // If the node's size is not determined by the children(ie, if it's not SizeKind::Fit), calculate how many fractional units fit in the node
        if axis == self.get(node).params.layout.axis && !matches!(self.get(node).params.size.on_axis(axis).size, SizeKind::Fit) {
            let basis_size = self.get(node).basis_size.on_axis(axis);
            let frac_units = self.get(node).frac_units.on_axis(axis);
            let space_left = (basis_size - non_frac_size - margin).max(0.0);
            let frac_size = space_left / frac_units.max(TINY);
            *self.get_mut(node).frac_units.on_axis_mut(axis) = basis_size / frac_size.max(TINY); 
        }

        // Calculate the final basis sizes for all the children
        let mut child = self.get(node).first_child;
        while child.is_some() {
            self.calc_up_dependent_basis_size(child, axis);
            child = self.get(child).next;
        }

        // Add the margins back to the basis size
        *self.get_mut(node).basis_size.on_axis_mut(axis) += margin;
    }

    fn calc_layout_main_axis(&mut self, node: UIRef, node_id: Id, total_space: Range, axis: Axis, memory: &mut Memory) {

        let layout = self.get(node).params.layout;
        let space = self.get(node).params.margin.apply_on_axis(total_space, axis);

        // The total basis size of all the children 
        let mut total_size = 0.0;
        // How many parts will the size violation be divided in?
        let mut violation_denominator = 0.0;
        // How many parts will the size underfill be divided in?
        let mut underfill_denominator = 0.0;

        let mut child_ref = self.get(node).first_child;
        while child_ref.is_some() {
            let child = self.get(child_ref);
            let size = child.basis_size.on_axis(axis);
            total_size += size;

            // If the child is allowed to shrink, it will share the violation
            if child.params.size.on_axis(axis).shrink {
                violation_denominator += size;
            }

            underfill_denominator += child.params.size.on_axis(axis).grow;
            child_ref = child.next;
        }

        *memory.get::<LayoutInfo>(node_id).children_base_size.on_axis_mut(axis) = total_size;

        let violation = if *layout.allow_overflow.on_axis(axis) { 0.0 } else { (total_size - space.size()).max(0.0) }; 
        let violation_denominator_inv = if violation_denominator < 0.00001 { 1.0 } else { 1.0 / violation_denominator };
        
        let underfill = (space.size() - total_size).max(0.0); 
        let underfill_denominator_inv = if underfill_denominator < 0.00001 { 1.0 } else { 1.0 / underfill_denominator }; 

        let mut offset = if underfill_denominator < 0.00001 {
            match layout.justify {
                Justify::Min => 0.0,
                Justify::Center => underfill / 2.0,
                Justify::Max => underfill,
            }
        } else {
            0.0
        };
        let mut child_ref = self.get(node).first_child;
        while child_ref.is_some() {
            let child = self.get_mut(child_ref);
            let shrink = if child.params.size.on_axis(axis).shrink {
                violation * child.basis_size.on_axis(axis) * violation_denominator_inv
            } else {
                0.0
            };
            let grow = child.params.size.on_axis(axis).grow * underfill * underfill_denominator_inv;
            let size = (child.basis_size.on_axis(axis) - shrink + grow).max(0.0); 
            let child_space = Range::min_size(space.min + offset, size);
            child.rect.set_axis_range(axis, child_space);
            self.calc_layout(child_ref, self.get(child_ref).id, child_space, axis, memory);
            child_ref = self.get(child_ref).next;
            offset += size;
        }

    }
    
    fn calc_layout_cross_axis(&mut self, node: UIRef, node_id: Id, total_space: Range, axis: Axis, memory: &mut Memory) {

        let layout = self.get(node).params.layout;
        let space = self.get(node).params.margin.apply_on_axis(total_space, axis);

        let mut child_base_size: f32 = 0.0;
        
        let mut child_ref = self.get(node).first_child;
        while child_ref.is_some() {
            let child = self.get_mut(child_ref);
            let size = child.basis_size.on_axis(axis);
            child_base_size = child_base_size.max(size);
            let size = if size < space.size() {
                if child.params.size.on_axis(axis).grow > 0.0 {
                    space.size()
                } else {
                    size
                }
            } else {
                if child.params.size.on_axis(axis).shrink && !layout.allow_overflow.on_axis(axis) {
                    space.size()
                } else {
                    size
                }
            };
            let child_space = match layout.align {
                Align::Min => Range::min_size(space.min, size),
                Align::Center => Range::center_size(space.center(), size),
                Align::Max => Range::max_size(space.max, size),
            };
            child.rect.set_axis_range(axis, child_space);
            self.calc_layout(child_ref, self.get(child_ref).id, child_space, axis, memory);
            child_ref = self.get(child_ref).next;
        }

        *memory.get::<LayoutInfo>(node_id).children_base_size.on_axis_mut(axis) = child_base_size;
    }

    fn calc_layout(&mut self, node: UIRef, node_id: Id, space: Range, axis: Axis, memory: &mut Memory) {
        if axis == self.get(node).params.layout.axis {
            self.calc_layout_main_axis(node, node_id, space, axis, memory);
        } else {
            self.calc_layout_cross_axis(node, node_id, space, axis, memory);
        }
    }

    fn calc_transformations(&mut self, node: UIRef, memory: &mut Memory, transform: TSTransform) {
        
        self.get_mut(node).transform = transform;

        let rect = self.get(node).rect;
        let id = self.get(node).id;
        memory.get::<LayoutInfo>(id).rect = rect;
        memory.get::<LayoutInfo>(id).screen_rect = transform * rect;

        let next_transform = transform * self.get(node).params.transform;
        let mut child_ref = self.get(node).first_child;
        while child_ref.is_some() {
            self.calc_transformations(child_ref, memory, next_transform); 
            child_ref = self.get(child_ref).next;
        }
    }

    pub(crate) fn layout(&mut self, space: Rect, memory: &mut Memory, text_resources: &mut TextResources) {

        for layer in self.layers.clone() {

            // Step 1: calculate down-dependent basis sizes
            for axis in AXES {
                self.calc_down_dependent_basis_size(memory, layer, axis, text_resources);
            }

            // Step 2: calculate up-dependent basis sizes
            for axis in AXES {
                self.calc_up_dependent_basis_size(layer, axis);
            }

            // Step 3: calculate layout
            for axis in AXES {
                self.calc_layout(layer, self.get(layer).id, space.axis_range(axis), axis, memory);
            }
            self.get_mut(layer).rect = space;

            // Step 4: apply transformations
            self.calc_transformations(layer, memory, TSTransform::IDENTITY);
        }

    }

    fn remember_node_layout(&self, node: UIRef, memory: &mut Memory) {
        let node = self.get(node);
        let layout_mem = memory.get::<LayoutMemory>(node.id);
        layout_mem.rect = node.rect;
        let screen_rect = node.transform * node.rect;
        layout_mem.screen_rect = screen_rect; 
        layout_mem.interaction_rect = screen_rect.grow(node.params.interaction_margin);
        layout_mem.transform = node.transform;
        layout_mem.first_child = node.first_child.as_option().map(|child| self.get(child).id);
        layout_mem.next = node.next.as_option().map(|next| self.get(next).id);
        layout_mem.sense_mouse = node.params.mouse;
        layout_mem.sense_scroll = node.params.scroll;
        layout_mem.has_interaction_priority = node.params.has_interaction_priority;

        let mut child = node.first_child;
        while child.is_some() {
            self.remember_node_layout(child, memory);
            child = self.get(child).next;
        }
    }

    pub(crate) fn remember_layout(&self, memory: &mut Memory) {
        memory.layer_ids = self.layers.iter().map(|layer| self.get(*layer).id).collect();
        for layer in &self.layers {
            self.remember_node_layout(*layer, memory);
        }
    }

}

pub(crate) struct LayoutMemory {
    /// The node's rectangle without transformations applied
    pub(crate) rect: Rect,
    /// The node's rectangle with transformations applied
    pub(crate) screen_rect: Rect,
    /// The rectangle where the node can receive mouse interaction
    pub(crate) interaction_rect: Rect,
    /// The full transformation applied to the node
    pub(crate) transform: TSTransform,
    pub(crate) first_child: Option<Id>,
    pub(crate) next: Option<Id>,

    pub(crate) sense_mouse: bool,
    pub(crate) sense_scroll: bool,
    pub(crate) has_interaction_priority: bool
}

impl Default for LayoutMemory {

    fn default() -> Self {
        Self {
            rect: Rect::ZERO,
            screen_rect: Rect::ZERO,
            interaction_rect: Rect::ZERO,
            transform: TSTransform::IDENTITY,
            first_child: None,
            next: None,
            sense_mouse: false,
            sense_scroll: false,
            has_interaction_priority: false
        }
    }

}

pub struct LayoutInfo {
    pub rect: Rect,
    pub screen_rect: Rect,
    pub children_base_size: Vec2
}

impl Default for LayoutInfo {

    fn default() -> Self {
        Self {
            rect: Rect::ZERO,
            screen_rect: Rect::ZERO,
            children_base_size: Vec2::ZERO
        }
    }

}

pub(crate) struct TextSizeCache {
    text: String,
    font_size: f32,
    line_height: f32,
    font: FontId,
    
    size: Vec2
}

impl Default for TextSizeCache {

    fn default() -> Self {
        Self {
            text: String::new(),
            font_size: 0.0,
            line_height: 0.0,
            size: Vec2::ZERO,
            font: FontId::default()
        }
    }

}
