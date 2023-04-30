use bevy::{
    prelude::*, reflect::TypeUuid, render::render_resource::AsBindGroup, sprite::Material2d,
};

// TODO: Generate id, currently is PBR_SHADER_HANDLE's id +1.
pub const UPSCALE_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 4805239651767701047);

// TODO: Generate id, currently is UPSCALE_SHADER_HANDLE's id +1.
pub const CELL_SHADE_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 4805239651767701048);

// TODO: Generate id, currently is CELL_SHADE_SHADER_HANDLE's id +1.
pub const BLUE_NOISE_64_IMAGE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Image::TYPE_UUID, 4805239651767701049);

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

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "480fd846-87f9-4e0f-ac5b-0dbb9761c31b"]
pub struct PixelPerfectCellShadeMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub dither_noise: Handle<Image>,
}

impl Default for PixelPerfectCellShadeMaterial {
    fn default() -> Self {
        Self {
            dither_noise: BLUE_NOISE_64_IMAGE_HANDLE.typed(),
        }
    }
}

impl Material for PixelPerfectCellShadeMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        CELL_SHADE_SHADER_HANDLE.typed().into()
    }
}
