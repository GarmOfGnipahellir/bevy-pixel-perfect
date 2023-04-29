use bevy::{
    prelude::*,
    render::render_resource::Extent3d,
    utils::HashSet,
    window::{PrimaryWindow, WindowCreated, WindowRef, WindowResized},
};

use crate::PixelPerfectBlitMaterial;

#[derive(Component, Debug, Clone, Reflect, FromReflect)]
#[reflect(Component, Default)]
pub struct PixelPerfectCamera {
    pub render_height: u32,
    pub window: WindowRef,
    pub render_target: Handle<Image>,
    pub blit_material: Handle<PixelPerfectBlitMaterial>,
}

impl Default for PixelPerfectCamera {
    fn default() -> Self {
        Self {
            render_height: 200,
            window: Default::default(),
            render_target: Default::default(),
            blit_material: Default::default(),
        }
    }
}

pub fn update_pixel_perfect_camera(
    mut window_resized_events: EventReader<WindowResized>,
    mut window_created_events: EventReader<WindowCreated>,
    primary_windows: Query<Entity, With<PrimaryWindow>>,
    windows: Query<&Window>,
    mut query: Query<&PixelPerfectCamera>,
    mut images: ResMut<Assets<Image>>,
    mut blit_materials: ResMut<Assets<PixelPerfectBlitMaterial>>,
) {
    let primary_window: Option<Entity> = primary_windows.iter().next();

    let mut changed_window_ids = HashSet::new();
    changed_window_ids.extend(window_created_events.iter().map(|event| event.window));
    changed_window_ids.extend(window_resized_events.iter().map(|event| event.window));

    for entry in query.iter_mut() {
        let camera: &PixelPerfectCamera = entry;

        let Some(normalized_window_ref) = camera.window.normalize(primary_window) else { continue; };
        if !changed_window_ids.contains(&normalized_window_ref.entity()) {
            continue;
        }
        let Ok(window): Result<&Window, _> = windows.get(normalized_window_ref.entity()) else { continue; };

        let pixel_width = (window.width() * camera.render_height as f32 / window.height()) as u32;
        let image: &mut Image = images.get_mut(&camera.render_target).unwrap();
        image.resize(Extent3d {
            width: pixel_width,
            height: camera.render_height,
            ..Default::default()
        });

        let blit_material: &mut PixelPerfectBlitMaterial =
            blit_materials.get_mut(&camera.blit_material).unwrap();
        blit_material.source_image = camera.render_target.clone();
    }
}
