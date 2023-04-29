use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_pixel_perfect::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelPerfectPlugin)
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut pp_commands: PixelPerfectCommands,
) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(1.0, 1.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("DamagedHelmet.glb#Scene0"),
        ..Default::default()
    });

    let mut source_camera = Camera::default();
    pp_commands.spawn_pixel_perfect_camera_setup(&mut source_camera, 200, 1);

    commands.spawn((
        Name::new("Camera 3D"),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(3.0),
                ..Default::default()
            }
            .into(),
            camera: source_camera,
            ..Default::default()
        },
    ));
}

fn rotate(mut query: Query<&mut Transform, With<Handle<Scene>>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        use std::f32::consts::TAU;

        let time = time.elapsed_seconds() * 0.5;
        let steps = 16.0;
        let y_angle = ((time * steps).floor() / steps) * TAU;
        transform.rotation = Quat::from_euler(EulerRot::YXZ, y_angle, 0.0, 0.0);
    }
}
