#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@group(1) @binding(2)
var<uniform> in_size: vec2<f32>;

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Bilinear upscale, reference impl:
    // https://github.com/rsn8887/Sharp-Bilinear-Shaders/blob/master/Copy_To_RetroPie/shaders/sharp-bilinear-simple.glsl

    let uv: vec2<f32> = coords_to_viewport_uv(position.xy, view.viewport);
    let out_size: vec2<f32> = view.viewport.zw;
    let texel = uv * in_size;
    let scale = max(floor(out_size / in_size), vec2<f32>(1.0, 1.0)) * 2.0;
    let region = 0.5 - 0.5 / scale;

    let s = fract(texel);
    let center_dist = s - 0.5;
    let f = (center_dist - clamp(center_dist, -region, region)) * scale + 0.5;
    let mod_texel = floor(texel) + f;

    var output_color = vec4<f32>(
        textureSample(texture, our_sampler, mod_texel / in_size).rgb,
        1.0
    );

    return output_color;
}