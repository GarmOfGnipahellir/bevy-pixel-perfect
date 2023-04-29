use bevy::{
    ecs::system::SystemParam,
    prelude::*,
    render::{
        camera::{RenderTarget, ScalingMode},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::ImageSampler,
        view::{Layer, RenderLayers},
    },
    sprite::MaterialMesh2dBundle,
};

use crate::{camera::PixelPerfectCamera, PixelPerfectBlitMaterial};

#[derive(SystemParam)]
pub struct PixelPerfectCommands<'w, 's> {
    commands: Commands<'w, 's>,
    meshes: ResMut<'w, Assets<Mesh>>,
    images: ResMut<'w, Assets<Image>>,
    blit_materials: ResMut<'w, Assets<PixelPerfectBlitMaterial>>,
}

impl<'w, 's> PixelPerfectCommands<'w, 's> {
    pub fn spawn_pixel_perfect_camera_setup(
        &mut self,
        source_camera: &mut Camera,
        render_height: u32,
        render_layer: Layer,
    ) {
        let RenderTarget::Window(window) = source_camera.target else {
            error!("Couldn't create pixel perfect camera setup, source camera aren't rendering to a window.");
            return;
        };

        let render_target: Handle<Image> = {
            let size = Extent3d {
                width: render_height,
                height: render_height,
                ..Default::default()
            };

            let mut image = Image {
                texture_descriptor: TextureDescriptor {
                    label: None,
                    size,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::Bgra8UnormSrgb,
                    mip_level_count: 1,
                    sample_count: 1,
                    usage: TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_DST
                        | TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                },
                sampler_descriptor: ImageSampler::nearest(),
                ..Default::default()
            };

            image.resize(size);

            self.images.add(image)
        };

        source_camera.target = RenderTarget::Image(render_target.clone());

        let blit_material: Handle<PixelPerfectBlitMaterial> =
            self.blit_materials.add(PixelPerfectBlitMaterial {
                source_image: render_target.clone(),
            });

        self.commands.spawn((
            Name::new("Pixel Perfect Upscale Mesh"),
            MaterialMesh2dBundle::<PixelPerfectBlitMaterial> {
                mesh: self
                    .meshes
                    .add(Mesh::from(shape::Quad::new(Vec2::new(2.0, 1.0))))
                    .into(),
                material: blit_material.clone(),
                ..Default::default()
            },
            RenderLayers::layer(render_layer),
        ));

        self.commands.spawn((
            Name::new("Pixel Perfect Upscale Camera"),
            Camera2dBundle {
                projection: OrthographicProjection {
                    scaling_mode: ScalingMode::FixedVertical(1.0),
                    ..Default::default()
                }
                .into(),
                camera: Camera {
                    order: source_camera.order + 1,
                    ..Default::default()
                },
                ..Default::default()
            },
            RenderLayers::layer(render_layer),
            PixelPerfectCamera {
                render_height,
                window,
                render_target,
                blit_material,
            },
        ));
    }
}
