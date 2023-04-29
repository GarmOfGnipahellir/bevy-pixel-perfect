use bevy::{
    prelude::*, reflect::TypeUuid, render::render_resource::AsBindGroup, sprite::Material2d,
};

// TODO: Generate id, currently is PBR_SHADER_HANDLE's id +1.
pub const UPSCALE_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 4805239651767701047);

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "9881d697-1be8-4fb9-9918-822dde73040f"]
pub struct PixelPerfectUpscaleMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,
    #[uniform(2)]
    pub input_size: Vec2,
}

impl Material2d for PixelPerfectUpscaleMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        UPSCALE_SHADER_HANDLE.typed().into()
    }
}
