pub mod axonometric;

use bevy::prelude::*;

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
    }
}
