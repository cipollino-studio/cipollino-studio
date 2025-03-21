
struct Uniforms {
    view_proj: mat4x4<f32>,
    canvas_size: vec2<f32>,
}

var<push_constant> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;

    let canvas_size = uniforms.canvas_size;

    let inner_tl = uniforms.view_proj * vec4(-canvas_size.x / 2.0,  canvas_size.y / 2.0, 0.0, 1.0);
    let inner_tr = uniforms.view_proj * vec4( canvas_size.x / 2.0,  canvas_size.y / 2.0, 0.0, 1.0);
    let inner_bl = uniforms.view_proj * vec4(-canvas_size.x / 2.0, -canvas_size.y / 2.0, 0.0, 1.0);
    let inner_br = uniforms.view_proj * vec4( canvas_size.x / 2.0, -canvas_size.y / 2.0, 0.0, 1.0);

    // Too lazy to use vertex buffers :P
    var verts = array(
        vec4(-1.0,  1.0, 0.0, 1.0),
        inner_tl,
        vec4( 1.0,  1.0, 0.0, 1.0),

        vec4( 1.0,  1.0, 0.0, 1.0),
        inner_tl,
        inner_tr,

        vec4( 1.0,  1.0, 0.0, 1.0),
        inner_tr,
        vec4( 1.0, -1.0, 0.0, 1.0),

        inner_tr,
        inner_br,
        vec4( 1.0, -1.0, 0.0, 1.0),

        inner_br,
        vec4(-1.0, -1.0, 0.0, 1.0),
        vec4( 1.0, -1.0, 0.0, 1.0),

        inner_bl,
        vec4(-1.0, -1.0, 0.0, 1.0),
        inner_br,

        vec4(-1.0,  1.0, 0.0, 1.0),
        vec4(-1.0, -1.0, 0.0, 1.0),
        inner_bl,

        vec4(-1.0,  1.0, 0.0, 1.0),
        inner_bl,
        inner_tl 
    );

    out.clip_position = verts[in_vertex_index];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 0.35);
}
