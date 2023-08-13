#import bevy_render::view View
#import bevy_render::globals Globals

@group(0) @binding(0)
var<uniform> view: View;
@group(0) @binding(1)
var<uniform> globals: Globals;

struct VertexOutput {
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(3) @interpolate(flat) mode: u32,
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vertex(
    @location(0) vertex_position: vec3<f32>,
    @location(1) vertex_uv: vec2<f32>,
    @location(2) vertex_color: vec4<f32>,
    @location(3) mode: u32,
) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vertex_uv;
    out.position = view.view_proj * vec4<f32>(vertex_position, 1.0);
    out.color = vertex_color;
    out.mode = mode;
    return out;
}

@group(1) @binding(0)
var sprite_texture: texture_2d<f32>;
@group(1) @binding(1)
var sprite_sampler: sampler;

const gradient_ease: f32 = 20.0;
const width = 0.1;
const PI = 3.141592656;
const TAU = 6.283185312;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let fill_amount = (sin(globals.time) + 1.0) / 2.0;
    let fill_angle = fill_amount * TAU;
    let uv = in.uv * 2.0 - 1.0;
    var color = vec4<f32>(0.0);
    if (atan2(uv.y, uv.x) + PI < fill_angle) {
        var inner_width = 1.0 - width;
        inner_width *= inner_width;
        let d = uv.x * uv.x + uv.y * uv.y;
        if (d <= 1.0 && d >= inner_width) {
            var w: f32 = abs((1.0 + inner_width) / 2.0 - d) / (1.0 - inner_width);
            w = 1.0 - pow(w + 0.5, gradient_ease);
            color = vec4<f32>(1.0, 1.0, 1.0, min(1.0, w));
        } else {
            color.a = 0.0;
        }
    } else {
        color.a = 0.0;
    }
    return color;

	
}
