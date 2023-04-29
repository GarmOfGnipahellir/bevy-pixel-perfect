use bevy::{
    prelude::*, reflect::TypeUuid, render::render_resource::AsBindGroup, sprite::Material2d,
};

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "9881d697-1be8-4fb9-9918-822dde73040f"]
pub struct PixelPerfectUpscaleMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,
}

impl Material2d for PixelPerfectUpscaleMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/upscale.wgsl".into()
    }
}
