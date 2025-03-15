
struct StampInput {
    @location(0) pos: vec2<f32>,
    @location(1) right: vec2<f32>,
}

struct StrokeUniforms {
    view_proj: mat4x4<f32>,
    color: vec4<f32>,
    resolution: vec2<f32>,
    padding: vec2<f32>
}

var<push_constant> uniforms: StrokeUniforms;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) coord: vec2<f32>
};

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;

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

    var uv = array(
        vec2(0.0, 0.0),
        vec2(1.0, 0.0),
        vec2(0.0, 1.0),

        vec2(0.0, 1.0),
        vec2(1.0, 0.0),
        vec2(1.0, 1.0),
    );
    out.uv = uv[in_vertex_index];

    out.coord = (out.clip_position.xy * 0.5 + 0.5) * uniforms.resolution;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let brush_alpha = textureSample(texture, texture_sampler, in.uv).x;
    return uniforms.color * vec4(1.0, 1.0, 1.0, brush_alpha);
}

@fragment
fn fs_picking(in: VertexOutput) -> @location(0) vec4<f32> {
    let brush_alpha = textureSample(texture, texture_sampler, in.uv).x;
    if brush_alpha < 1 / 256.0 {
        discard;
    }
    return uniforms.color;
}

@fragment
fn fs_selection(in: VertexOutput) -> @location(0) vec4<f32> {
    let brush_alpha = textureSample(texture, texture_sampler, in.uv).x;
    if brush_alpha < 1 / 256.0 {
        discard;
    }

    let x = i32(in.coord.x); 
    let y = i32(in.coord.y); 
    if (x / 2 + y / 2) % 2 == 0 {
        discard;
    }
    return uniforms.color;
}
