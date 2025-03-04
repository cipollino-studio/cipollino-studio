
struct StampInput {
    @location(0) pos: vec2<f32>,
    @location(1) right: vec2<f32>
}

struct Camera {
    view_proj: mat4x4<f32> 
}

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
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
    out.clip_position = camera.view_proj * vec4(stamp.pos + rect_vertex_offset[in_vertex_index], 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
