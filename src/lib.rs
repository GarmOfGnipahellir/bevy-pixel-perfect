pub mod axonometric;
pub mod camera;
pub mod commands;
pub mod materials;

use bevy::{
    asset::{load_internal_asset, load_internal_binary_asset},
    prelude::*,
    render::texture::{CompressedImageFormats, ImageType},
    sprite::Material2dPlugin,
};

use crate::{
    camera::{update_pixel_perfect_camera, PixelPerfectCamera},
    materials::*,
};

pub mod prelude {
    pub use crate::camera::PixelPerfectCamera;
    pub use crate::commands::PixelPerfectCommands;
    pub use crate::materials::{PixelPerfectCellShadeMaterial, PixelPerfectUpscaleMaterial};
    pub use crate::PixelPerfectPlugin;
}

pub struct PixelPerfectPlugin;

impl Plugin for PixelPerfectPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            UPSCALE_SHADER_HANDLE,
            "upscale.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            CELL_SHADE_SHADER_HANDLE,
            "cell_shade.wgsl",
            Shader::from_wgsl
        );
        load_internal_binary_asset!(
            app,
            BLUE_NOISE_64_IMAGE_HANDLE,
            "BlueNoise_64.png",
            |b: &[u8]| {
                Image::from_buffer(
                    b,
                    ImageType::Extension("png"),
                    CompressedImageFormats::all(),
                    true,
                )
                .unwrap()
            }
        );

        // NOTE: Can feature gate this
        {
            use bevy::{
                prelude::*,
                render::{
                    camera::{camera_system, CameraProjectionPlugin},
                    view::{update_frusta, VisibilitySystems},
                },
                transform::TransformSystem,
            };

            use axonometric::AxonometricProjection;

            app.add_plugin(CameraProjectionPlugin::<AxonometricProjection>::default())
                .add_system(
                    update_frusta::<AxonometricProjection>
                        .in_set(VisibilitySystems::UpdateProjectionFrusta)
                        .after(camera_system::<AxonometricProjection>)
                        .after(TransformSystem::TransformPropagate)
                        // We assume that no camera will have more than one projection component,
                        // so these systems will run independently of one another.
                        // FIXME: Add an archetype invariant for this https://github.com/bevyengine/bevy/issues/1481.
                        .ambiguous_with(update_frusta::<OrthographicProjection>)
                        .ambiguous_with(update_frusta::<PerspectiveProjection>)
                        .ambiguous_with(update_frusta::<Projection>),
                );
        }

        app.register_type::<PixelPerfectCamera>()
            .add_plugin(Material2dPlugin::<PixelPerfectUpscaleMaterial>::default())
            .add_plugin(MaterialPlugin::<PixelPerfectCellShadeMaterial>::default())
            .add_system(update_pixel_perfect_camera.in_base_set(CoreSet::PostUpdate));

        let dot = bevy_mod_debugdump::render_graph_dot(
            app,
            &bevy_mod_debugdump::render_graph::Settings::default(),
        );
        std::fs::write("render-graph.dot", dot).unwrap();
    }
}
