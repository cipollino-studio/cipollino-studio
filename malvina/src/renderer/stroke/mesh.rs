use crate::Stroke;


#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct StrokeStampInstance {
    /// The position of the stamp
    pub pos: [f32; 2],
    /// The vector pointing to the right, relative to the stamps rotation.
    /// The magnitude is equal to the stamp's radius(half the width/height of the quad) 
    pub right: [f32; 2] 
}

impl StrokeStampInstance {

    pub const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2, // pos
        1 => Float32x2  // right
    ];

    pub const DESC: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &Self::ATTRIBS,
    };

}

pub struct StrokeMesh {
    pub(crate) instance_buffer: wgpu::Buffer,
    pub(crate) n_stamps: u32,
}

impl StrokeMesh {

    pub fn new(device: &wgpu::Device, stroke: &Stroke) -> Self {
        use wgpu::util::DeviceExt;

        let stamps = stroke.meshgen(); 

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("malvina_stroke_buffer"),
            contents: bytemuck::cast_slice(&stamps),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::VERTEX,
        });

        Self {
            instance_buffer: buffer,
            n_stamps: stamps.len() as u32
        }
    }

}
