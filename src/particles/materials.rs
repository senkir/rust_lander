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
        "materials/flame_jet.wesl".into()
    }
}

#[derive(AsBindGroup, Asset, TypePath, Clone)]
pub struct SideJetMaterial {
    #[uniform(0)]
    pub tint: LinearRgba,
}

impl Default for SideJetMaterial {
    fn default() -> Self {
        Self { tint: LinearRgba::new(1.0, 1.0, 1.0, 1.0) }
    }
}

impl Particle2dMaterial for SideJetMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "materials/arrow_jet.wesl".into()
    }
}