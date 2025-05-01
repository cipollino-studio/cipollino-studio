
struct Vertex {
    @location(0) pos: vec2<f32>
}

struct FillUniforms {
    trans: mat4x4<f32>,
    view_proj: mat4x4<f32>,
    color: vec4<f32>,
    resolution: vec2<f32>
}

var<push_constant> uniforms: FillUniforms;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) coord: vec2<f32>
}

@vertex
fn vs_main(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = uniforms.view_proj * uniforms.trans * vec4(vertex.pos, 0.0, 1.0);
    out.coord = (out.clip_position.xy * 0.5 + 0.5) * uniforms.resolution;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return uniforms.color;
}

@fragment
fn fs_selection(in: VertexOutput) -> @location(0) vec4<f32> {
    let x = i32(in.coord.x); 
    let y = i32(in.coord.y); 
    if (x / 2 + y / 2) % 2 == 0 {
        discard;
    }
    return uniforms.color;
}