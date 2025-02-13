
struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0)  color: vec4<f32>,
    @location(1)  uv: vec2<f32>,
    @location(2)  tex: u32,
    @location(3)  pos: vec2<f32>,
    @location(4)  clip_min: vec2<f32>,
    @location(5)  clip_max: vec2<f32>,
    @location(6)  rect_center: vec2<f32>,
    @location(7)  rect_half_size: vec2<f32>,
    @location(8)  rounding: f32,
    @location(9)  stroke_color: vec4<f32>,
    @location(10) stroke_width: f32
};

struct RectData {
    @location(0)  min: vec2<f32>,
    @location(1)  size: vec2<f32>,
    @location(2)  uv_min: vec2<f32>,
    @location(3)  uv_size: vec2<f32>,
    @location(4)  color: vec4<f32>,
    @location(5)  tex: u32,
    @location(6)  clip_min: vec2<f32>,
    @location(7)  clip_max: vec2<f32>,
    @location(8)  rounding: f32,
    @location(9)  stroke_color: vec4<f32>,
    @location(10) stroke_width: f32
}

struct Uniforms {
    screen_size: vec2<f32>
}

@group(0) @binding(1)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(
    @builtin(vertex_index) vert_id: u32,
    rect: RectData
) -> VertexOutput {
    var out: VertexOutput;

    var RECT_SIZE_SCALE = array(
        vec2(0.0,  0.0),
        vec2(0.0,  1.0),
        vec2(1.0,  0.0),

        vec2(0.0,  1.0),
        vec2(1.0,  1.0),
        vec2(1.0,  0.0),
    );

    let pos = rect.min + RECT_SIZE_SCALE[vert_id] * rect.size; 
    out.clip_pos = vec4<f32>(2.0 * pos.x / uniforms.screen_size.x - 1.0, -2.0 * pos.y / uniforms.screen_size.y + 1.0, 0.0, 1.0);
    out.color = rect.color;
    out.uv = rect.uv_min + RECT_SIZE_SCALE[vert_id] * rect.uv_size;
    out.tex = rect.tex;
    out.pos = pos;
    out.clip_min = rect.clip_min;
    out.clip_max = rect.clip_max;
    out.rect_center = rect.min + rect.size * 0.5;
    out.rect_half_size = rect.size * 0.5;
    out.rounding = rect.rounding;
    out.stroke_color = rect.stroke_color;
    out.stroke_width = rect.stroke_width;
    return out;
}

@group(0) @binding(0)
var s: sampler;
@group(0) @binding(2)
var tex1: texture_2d<f32>;
@group(0) @binding(3)
var tex2: texture_2d<f32>;
@group(0) @binding(4)
var tex3: texture_2d<f32>;
@group(0) @binding(5)
var tex4: texture_2d<f32>;
@group(0) @binding(6)
var tex5: texture_2d<f32>;
@group(0) @binding(7)
var tex6: texture_2d<f32>;
@group(0) @binding(8)
var tex7: texture_2d<f32>;
@group(0) @binding(9)
var tex8: texture_2d<f32>;

fn sample(uv: vec2<f32>, tex: u32) -> vec4<f32> {
    switch(tex) {
        case 0u, default: {
            return vec4(1.0, 1.0, 1.0, 1.0); 
        }
        case 1u: {
            return textureSample(tex1, s, uv);
        }
        case 2u: {
            return textureSample(tex2, s, uv);
        }
        case 3u: {
            return textureSample(tex3, s, uv);
        }
        case 4u: {
            return textureSample(tex4, s, uv);
        }
        case 5u: {
            return textureSample(tex5, s, uv);
        }
        case 6u: {
            return textureSample(tex6, s, uv);
        }
        case 7u: {
            return textureSample(tex7, s, uv);
        }
        case 8u: {
            return textureSample(tex8, s, uv);
        }
    }
}

fn rounded_rect_sdf(pos: vec2<f32>, rect_center: vec2<f32>, rect_half_size: vec2<f32>, r: f32) -> f32 {
    let d2 = abs(rect_center - pos) - rect_half_size + vec2(r, r);
    return min(max(d2.x, d2.y), 0.0) + length(max(d2, vec2(0.0, 0.0))) - r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if
        in.pos.x < in.clip_min.x ||
        in.pos.y < in.clip_min.y ||
        in.pos.x > in.clip_max.x ||
        in.pos.y > in.clip_max.y
    {
        discard;         
    }
    
    let outer_sdf = rounded_rect_sdf(in.pos, in.rect_center, in.rect_half_size, in.rounding);
    let rounding_factor = vec4(1.0, 1.0, 1.0, 1.0 - smoothstep(0.0, 1.0, outer_sdf));

    var color = in.color;
    if in.stroke_width > 0.0 {
        let inner_half_size = in.rect_half_size - vec2(in.stroke_width, in.stroke_width);
        let inner_rounding = max(in.rounding - 2.0 * in.stroke_width, 0.0);
        let inner_sdf = rounded_rect_sdf(in.pos, in.rect_center, inner_half_size, inner_rounding);
        let stroke_fac = smoothstep(-0.5, 0.5, inner_sdf);
        color = mix(color, in.stroke_color, stroke_fac);
    }
    
    return color * sample(in.uv, in.tex) * rounding_factor;
}
