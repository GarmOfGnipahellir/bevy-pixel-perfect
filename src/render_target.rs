use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::ImageSampler,
    },
    utils::HashSet,
    window::{PrimaryWindow, WindowCreated, WindowRef, WindowResized},
};

use crate::PixelPerfectBlitMaterial;

#[derive(Component, Debug, Clone, Reflect, FromReflect)]
#[reflect(Component)]
pub struct PixelPerfectRenderTarget {
    pub image: Handle<Image>,
    pub pixel_height: u32,
    pub window: WindowRef,
}

impl FromWorld for PixelPerfectRenderTarget {
    fn from_world(world: &mut World) -> Self {
        let mut images = world.resource_mut::<Assets<Image>>();
        Self::new(200, &mut images)
    }
}

impl PixelPerfectRenderTarget {
    pub fn new(pixel_height: u32, images: &mut Assets<Image>) -> Self {
        let size = Extent3d {
            width: pixel_height,
            height: pixel_height,
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

        Self {
            image: images.add(image),
            pixel_height,
            window: WindowRef::Primary,
        }
    }
}

pub fn update_render_target(
    mut window_resized_events: EventReader<WindowResized>,
    mut window_created_events: EventReader<WindowCreated>,
    primary_windows: Query<Entity, With<PrimaryWindow>>,
    windows: Query<&Window>,
    mut query: Query<&PixelPerfectRenderTarget>,
    mut images: ResMut<Assets<Image>>,
    mut blits: ResMut<Assets<PixelPerfectBlitMaterial>>,
) {
    let primary_window: Option<Entity> = primary_windows.iter().next();

    let mut changed_window_ids = HashSet::new();
    changed_window_ids.extend(window_created_events.iter().map(|event| event.window));
    changed_window_ids.extend(window_resized_events.iter().map(|event| event.window));

    for entry in query.iter_mut() {
        let rt: &PixelPerfectRenderTarget = entry;

        let Some(normalized_window_ref) = rt.window.normalize(primary_window) else { continue; };
        if !changed_window_ids.contains(&normalized_window_ref.entity()) {
            continue;
        }
        let Ok(window): Result<&Window, _> = windows.get(normalized_window_ref.entity()) else { continue; };

        let pixel_width = (window.width() * rt.pixel_height as f32 / window.height()) as u32;
        let image: &mut Image = images.get_mut(&rt.image).unwrap();
        image.resize(Extent3d {
            width: pixel_width,
            height: rt.pixel_height,
            ..Default::default()
        });

        // FIXME: Hacky way to update all blit materials, assumes only ONE pixel perfect setup
        for blit in blits.iter_mut() {
            blit.1.source_image = rt.image.clone();
        }
    }
}
