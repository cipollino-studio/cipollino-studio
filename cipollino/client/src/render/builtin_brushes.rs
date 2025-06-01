
pub struct BuiltinBrush {
    pub name: &'static str,
    pub brush: malvina::BrushSettings,
    texture: &'static [u8]
}

pub const BUILTIN_BRUSHES: &[BuiltinBrush] = &[
    BuiltinBrush {
        name: "Hard Round",
        brush: malvina::BrushSettings::new(),
        texture: include_bytes!("../../res/brushes/circle.png")
    },
    BuiltinBrush {
        name: "Soft Round",
        brush: malvina::BrushSettings::new(),
        texture: include_bytes!("../../res/brushes/soft.png")
    },
    BuiltinBrush {
        name: "Spray",
        brush: malvina::BrushSettings::new().random_angle().with_stamp_spacing(3.0),
        texture: include_bytes!("../../res/brushes/faint-spray.png")
    },
    BuiltinBrush {
        name: "Charcoal",
        brush: malvina::BrushSettings::new().with_stamp_spacing(4.0).random_angle(),
        texture: include_bytes!("../../res/brushes/dots.png")
    },
    BuiltinBrush {
        name: "Hatch",
        brush: malvina::BrushSettings::new().with_stamp_spacing(3.0).with_base_angle(90.0).with_angle_range(60.0).with_shift_range(0.4),
        texture: include_bytes!("../../res/brushes/lines.png")
    }
];

pub struct BuiltinBrushTextures {
    pub textures: Vec<malvina::BrushTexture>,
    pub previews: Vec<pierro::Texture>
}

impl BuiltinBrushTextures {

    pub fn load(device: &pierro::wgpu::Device, queue: &pierro::wgpu::Queue, renderer: &mut malvina::Renderer) -> Self {
        let brush_texture_resources = malvina::BrushTextureResources::new(device); 
        let mut textures = Vec::new();
        for brush in BUILTIN_BRUSHES {
            let image = pierro::image::load_from_memory(brush.texture).unwrap();
            let pixels = image.as_bytes();
            let step = pixels.len() / (image.width() * image.height()) as usize;
            let mut data = Vec::new(); 
            for pixel in pixels.iter().step_by(step) { 
                data.push(*pixel);
            }
            textures.push(malvina::BrushTexture::new(device, queue, &brush_texture_resources, image.width(), image.height(), &data));
        }

        let mut previews = Vec::new();
        let mut preview_path = malvina::Stroke::empty();
        preview_path.path.pts.push(elic::BezierPoint {
            prev: malvina::StrokePoint {
                pt: elic::vec2(-150.0, -40.0),
                pressure: 0.0,
            },
            pt: malvina::StrokePoint {
                pt: elic::vec2(-100.0, 0.0),
                pressure: 0.0
            },
            next: malvina::StrokePoint { 
                pt: elic::vec2(-50.0, 40.0),
                pressure: 3.0
            },
        });
        preview_path.path.pts.push(elic::BezierPoint {
            prev: malvina::StrokePoint {
                pt: elic::vec2(50.0, -40.0),
                pressure: 3.0,
            },
            pt: malvina::StrokePoint {
                pt: elic::vec2(100.0, 0.0),
                pressure: 0.0
            },
            next: malvina::StrokePoint { 
                pt: elic::vec2(150.0, 40.0),
                pressure: 0.0
            },
        });
        for (idx, brush) in BUILTIN_BRUSHES.iter().enumerate() {
            let mesh = malvina::StrokeMesh::new(device, &preview_path, 10.0, &brush.brush);
            let texture = pierro::Texture::create_render_texture(device, 600, 60);
            renderer.render(device, queue, texture.texture(), malvina::Camera::new(elic::Vec2::ZERO, 0.8), elic::Color::WHITE, 1.0, |rndr| {
                rndr.render_stroke(&mesh, elic::Color::BLACK, elic::Mat4::IDENTITY, Some(&textures[idx]));
            });
            previews.push(texture);
        }

        Self {
            textures,
            previews
        }
    }

}
