pub mod axonometric;
pub mod camera;
pub mod render_target;

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::AsBindGroup,
    sprite::{Material2d, Material2dPlugin},
};

use crate::render_target::{update_render_target, PixelPerfectRenderTarget};

pub mod prelude {
    pub use crate::render_target::PixelPerfectRenderTarget;
    pub use crate::{PixelPerfectBlitMaterial, PixelPerfectPlugin};
}

pub struct PixelPerfectPlugin;

impl Plugin for PixelPerfectPlugin {
    fn build(&self, app: &mut App) {
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

        app.register_type::<PixelPerfectRenderTarget>()
            .add_plugin(Material2dPlugin::<PixelPerfectBlitMaterial>::default())
            .add_system(update_render_target.in_base_set(CoreSet::PostUpdate));

        let dot = bevy_mod_debugdump::render_graph_dot(
            app,
            &bevy_mod_debugdump::render_graph::Settings::default(),
        );
        std::fs::write("render-graph.dot", dot).unwrap();
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "9881d697-1be8-4fb9-9918-822dde73040f"]
pub struct PixelPerfectBlitMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,
}

impl Material2d for PixelPerfectBlitMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/blit.wgsl".into()
    }
}
