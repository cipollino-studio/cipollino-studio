use crate::FillPaths;


#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct FillVertex {
    pub pos: [f32; 2],
}

impl FillVertex {

    pub const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![
        0 => Float32x2, // pos
    ];

    pub const DESC: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &Self::ATTRIBS,
    };

}

pub struct FillMesh {
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) n_verts: u32
}

impl FillMesh {

    pub fn new(device: &wgpu::Device, fill: &FillPaths) -> Self {
        use wgpu::util::DeviceExt;

        let verts = fill.meshgen();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("malvina_fill_mesh"),
            contents: bytemuck::cast_slice(&verts),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::VERTEX
        });

        Self {
            vertex_buffer: buffer,
            n_verts: verts.len() as u32,
        }
    }

}
