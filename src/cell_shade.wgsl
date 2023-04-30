#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting

@group(1) @binding(0)
var dither_texture: texture_2d<f32>;
@group(1) @binding(1)
var dither_sampler: sampler;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

fn remap_pdf_tri_unity(v: f32) -> f32 {
    var v = v * 2.0 - 1.0;
    v = sign(v) * (1.0 - sqrt(1.0 - abs(v)));

    //return v; //note: [-1;1[
    return 0.5 + 0.5 * v; //note: [0;1[
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let view_z = dot(vec4<f32>(
        view.inverse_view[0].z,
        view.inverse_view[1].z,
        view.inverse_view[2].z,
        view.inverse_view[3].z
    ), in.world_position);
    let cluster_index = fragment_cluster_index(in.frag_coord.xy, view_z, view.projection[3].w == 1.0);
    let offset_and_counts = unpack_offset_and_counts(cluster_index);

    var direct_light = 0.0;

    let steps = 3.0;
    let n_directional_lights = lights.n_directional_lights;
    for (var i: u32 = 0u; i < n_directional_lights; i = i + 1u) {
        let ndotl: f32 = dot(in.world_normal, lights.directional_lights[i].direction_to_light) * 0.5 + 0.5;
        // direct_light += floor(ndotl * steps) / (steps - 1.0);
        direct_light += ndotl;
    }

    var dither_size = 64.0;
    var dither = textureSample(dither_texture, dither_sampler, fract(in.frag_coord.xy / dither_size));
    var dither_tri = remap_pdf_tri_unity(dither.r);

    var dither_direct_light = direct_light + (dither_tri - 0.5) / steps;
    dither_direct_light = floor(dither_direct_light * steps) / (steps - 1.0);

    return vec4<f32>(vec3<f32>(dither_direct_light), 1.0);
}