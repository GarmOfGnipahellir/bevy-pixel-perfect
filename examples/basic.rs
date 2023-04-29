use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_pixel_perfect::PixelPerfectPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelPerfectPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn(CameraAxonometricBundle {
    //     transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     projection: AxonometricProjection {
    //         height: 3.0,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(3.0),
            ..Default::default()
        }
        .into(),
        ..Default::default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(1.0, 1.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("DamagedHelmet.glb#Scene0"),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(2.0, 0.0, 0.0),
        mesh: meshes.add(shape::Box::new(1.0, 2.0, 1.0).into()),
        material: materials.add(Color::WHITE.into()),
        ..Default::default()
    });
}
