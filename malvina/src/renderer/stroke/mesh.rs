
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct StrokeStampInstance {
    /// The position of the stamp
    pos: glam::Vec2,
    /// The vector pointing to the right, relative to the stamps rotation.
    /// The magnitude is equal to the stamp's radius(half the width/height of the quad) 
    right: glam::Vec2
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

    pub fn new(device: &wgpu::Device) -> Self {
        use wgpu::util::DeviceExt;

        let mut stamps = Vec::new();
        let n = 1880;
        for i in 0..n {
            let t = (i as f32) / (n as f32);
            let angle = std::f32::consts::PI * t; 
            stamps.push(StrokeStampInstance {
                pos: glam::vec2(angle.cos(), angle.sin()) * 300.0,
                right: glam::vec2(angle.sin(), -angle.cos()) * 5.0
            });
        }

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
