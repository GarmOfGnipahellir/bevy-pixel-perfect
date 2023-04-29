#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = coords_to_viewport_uv(position.xy, view.viewport);

    // Should implement a bilinear upscale, found this reference impl for Unity:
    // https://github.com/AlexanderOcias/UnityPixelArtCamera/blob/master/Assets/Ocias/PixelArtCamera/BilinearSharp.shader

    // Sample each color channel with an arbitrary shift
    var output_color = vec4<f32>(
        textureSample(texture, our_sampler, uv).rgb,
        1.0
    );

    return output_color;
}