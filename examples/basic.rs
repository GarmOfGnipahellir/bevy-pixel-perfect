use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::{RenderTarget, ScalingMode},
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
};
use bevy_pixel_perfect::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelPerfectPlugin)
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut blit_materails: ResMut<Assets<PixelPerfectBlitMaterial>>,
) {
    let rt = PixelPerfectRenderTarget::new(200, &mut images);
    let layer = RenderLayers::layer(1);

    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(2.0, 0.0, 0.0),
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 2.0, 1.0))),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(rt.image.clone()),
                ..Default::default()
            }),
            ..Default::default()
        },
        layer,
    ));

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(1.0, 1.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("DamagedHelmet.glb#Scene0"),
        ..Default::default()
    });

    commands.spawn((
        MaterialMesh2dBundle::<PixelPerfectBlitMaterial> {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(6.0, 6.0))))
                .into(),
            material: blit_materails.add(PixelPerfectBlitMaterial {
                source_image: rt.image.clone(),
            }),
            ..Default::default()
        },
        layer,
    ));

    commands.spawn((
        Name::new("Pixel Perfect Camera "),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(3.0),
                ..Default::default()
            }
            .into(),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::RED),
                ..Default::default()
            },
            camera: Camera {
                target: RenderTarget::Image(rt.image.clone()),
                ..Default::default()
            },
            ..Default::default()
        },
        rt,
    ));

    commands.spawn((
        Name::new("Main Camera"),
        Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(3.0),
                ..Default::default()
            }
            .into(),
            camera: Camera {
                order: 1,
                ..Default::default()
            },
            ..Default::default()
        },
        layer,
    ));
}
