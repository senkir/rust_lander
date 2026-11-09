use bevy::asset::{Asset, Handle};
use bevy::image::Image;
use bevy::prelude::{LinearRgba, TypePath};
use bevy::render::render_resource::AsBindGroup;
use bevy_enoki::prelude::Particle2dMaterial;

#[derive(AsBindGroup, Asset, TypePath, Clone, Default)]
pub struct FireParticleMaterial {
    #[uniform(0)] pub color: LinearRgba,
}

impl Particle2dMaterial for FireParticleMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "sample_mat.wesl".into()
    }
}