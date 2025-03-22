
struct Uniforms {
    view_proj: mat4x4<f32>,
    color: vec4<f32>,
    a: vec2<f32>,
    b: vec2<f32>,
    r: f32,
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
    
    let r = normalize(uniforms.b - uniforms.a);
    let u = vec2(-r.y, r.x) * uniforms.r; 
    var rect_vertex_offset = array(
        uniforms.a - u,
        uniforms.b - u,
        uniforms.a + u,

        uniforms.a + u,
        uniforms.b - u,
        uniforms.b + u,
    );
    out.clip_position = uniforms.view_proj * vec4(rect_vertex_offset[in_vertex_index], 0.0, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return uniforms.color; 
}
