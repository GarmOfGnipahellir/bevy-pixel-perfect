use bevy::{
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    prelude::*,
    render::{
        camera::{camera_system, CameraProjection, CameraProjectionPlugin, CameraRenderGraph},
        primitives::Frustum,
        view::{update_frusta, ColorGrading, VisibilitySystems, VisibleEntities},
    },
    transform::TransformSystem,
};

pub struct PixelPerfectPlugin;

impl Plugin for PixelPerfectPlugin {
    fn build(&self, app: &mut App) {
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

const WIDTH_SCALE: f32 = 1.4142135624;

#[derive(Component, Debug, Clone, Reflect, FromReflect)]
#[reflect(Component, Default)]
pub struct AxonometricProjection {
    pub near: f32,
    pub far: f32,
    pub height: f32,
    pub aspect_ratio: f32,
}

impl CameraProjection for AxonometricProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        let half_height = self.height * 0.5;
        let half_width = self.aspect_ratio * half_height * WIDTH_SCALE;
        Mat4::orthographic_rh(
            -half_width,
            half_width,
            -half_height,
            half_height,
            // NOTE: near and far are swapped to invert the depth range from [0,1] to [1,0]
            // This is for interoperability with pipelines using infinite reverse perspective projections.
            self.far,
            self.near,
        )
    }

    fn update(&mut self, width: f32, height: f32) {
        self.aspect_ratio = width / height;
    }

    fn far(&self) -> f32 {
        self.far
    }
}

impl Default for AxonometricProjection {
    fn default() -> Self {
        Self {
            near: 0.0,
            far: 1000.0,
            height: 1.0,
            aspect_ratio: 1.0,
        }
    }
}

#[derive(Bundle)]
pub struct CameraAxonometricBundle {
    pub camera: Camera,
    pub camera_render_graph: CameraRenderGraph,
    pub projection: AxonometricProjection,
    pub visible_entities: VisibleEntities,
    pub frustum: Frustum,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub camera_3d: Camera3d,
    pub tonemapping: Tonemapping,
    pub dither: DebandDither,
    pub color_grading: ColorGrading,
}

impl Default for CameraAxonometricBundle {
    fn default() -> Self {
        Self {
            camera_render_graph: CameraRenderGraph::new(bevy::core_pipeline::core_3d::graph::NAME),
            camera: Default::default(),
            projection: Default::default(),
            visible_entities: Default::default(),
            frustum: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            camera_3d: Default::default(),
            tonemapping: Tonemapping::ReinhardLuminance,
            dither: DebandDither::Enabled,
            color_grading: ColorGrading::default(),
        }
    }
}
