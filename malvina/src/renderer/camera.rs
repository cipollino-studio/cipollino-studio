
pub struct Camera {
    center: glam::Vec2,
    zoom: f32
}

impl Camera {

    pub fn new(center_x: f32, center_y: f32, zoom: f32) -> Self {
        Camera {
            center: glam::vec2(center_x, center_y),
            zoom: zoom,
        }
    }

    pub(crate) fn calc_view_proj(&self, resolution: glam::Vec2) -> glam::Mat4 {
        let min = (self.center - resolution * 0.5) / self.zoom;
        let max = (self.center + resolution * 0.5) / self.zoom;
        glam::Mat4::orthographic_rh(min.x, max.x, min.y, max.y, -1.0, 1.0)
    }

}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct CameraUniforms { 
    pub view_proj: glam::Mat4
}

pub(crate) struct CameraUniformsBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup
}

impl CameraUniformsBuffer {

    pub fn new(device: &wgpu::Device) -> Self {

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("malvina_camera_buffer"),
            size: size_of::<CameraUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("malvina_camera_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None,
                }
            ] 
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("malvina_camera_bind_group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
        });

        Self {
            buffer,
            bind_group_layout,
            bind_group
        } 
    }

    pub fn update(&self, queue: &wgpu::Queue, camera: &Camera, resolution: glam::Vec2) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[CameraUniforms {
            view_proj: camera.calc_view_proj(resolution),
        }]));
    }

}
