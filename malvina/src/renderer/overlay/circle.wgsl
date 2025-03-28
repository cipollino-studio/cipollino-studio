
struct Uniforms {
    view_proj: mat4x4<f32>,
    color: vec4<f32>,
    pos: vec2<f32>,
    r: f32,
}

var<push_constant> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) pos: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    
    var rect_vertex_offset = array(
        uniforms.pos + vec2(-uniforms.r, -uniforms.r),
        uniforms.pos + vec2( uniforms.r, -uniforms.r),
        uniforms.pos + vec2(-uniforms.r,  uniforms.r),

        uniforms.pos + vec2( uniforms.r,  uniforms.r),
        uniforms.pos + vec2( uniforms.r, -uniforms.r),
        uniforms.pos + vec2(-uniforms.r,  uniforms.r),
    );
    let pos = rect_vertex_offset[in_vertex_index];
    out.clip_position = uniforms.view_proj * vec4(pos, 0.0, 1.0);
    out.pos = pos;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if length(in.pos - uniforms.pos) > uniforms.r {
        discard;
    }
    return uniforms.color; 
}
