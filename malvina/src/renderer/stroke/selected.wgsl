
struct StampInput {
    @location(0) pos: vec2<f32>,
    @location(1) right: vec2<f32>
}

struct StrokeUniforms {
    view_proj: mat4x4<f32>,
    resolution: vec2<f32>,
    color: f32,
    padding: f32
}

var<push_constant> uniforms: StrokeUniforms;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) coord: vec2<f32>
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    stamp: StampInput
) -> VertexOutput {
    var out: VertexOutput;
    
    let r = stamp.right;
    let l = -r;
    let u = vec2(-r.y, r.x);
    let d = -u;
    var rect_vertex_offset = array(
        l + d,
        r + d,
        l + u,

        l + u,
        r + d,
        r + u,
    );
    out.clip_position = uniforms.view_proj * vec4(stamp.pos + rect_vertex_offset[in_vertex_index], 0.0, 1.0);
    out.coord = (out.clip_position.xy * 0.5 + 0.5) * uniforms.resolution;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let x = i32(in.coord.x); 
    let y = i32(in.coord.y); 
    if (x / 2 + y / 2) % 2 == 0 {
        discard;
    }
    return vec4(uniforms.color, uniforms.color, uniforms.color, 1.0);
}
