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

#[derive(AsBindGroup, Asset, TypePath, Clone, Default)]
pub struct SmokeParticleMaterial {
    #[uniform(0)] pub color: LinearRgba,
}

impl Particle2dMaterial for SmokeParticleMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "materials/smoke_shader.wesl".into()
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

#[derive(AsBindGroup, Asset, TypePath, Clone)]
pub struct TractorBeamMaterial {
    #[uniform(0)]
    pub tint: LinearRgba,
}

impl Default for TractorBeamMaterial {
    fn default() -> Self {
        Self { tint: LinearRgba::new(1.0, 1.0, 1.0, 1.0) }
    }
}

impl Particle2dMaterial for TractorBeamMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "materials/tractor_beam.wesl".into()
    }
}

#[derive(AsBindGroup, Asset, TypePath, Clone)]
pub struct TractorSparkleMaterial {
    #[uniform(0)]
    pub tint: LinearRgba,
}


impl Default for TractorSparkleMaterial {
    fn default() -> Self {
        Self { tint: LinearRgba::new(1.0, 1.0, 1.0, 1.0) }
    }
}

impl Particle2dMaterial for TractorSparkleMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "materials/tractor_beam_sparkles.wesl".into()
    }
}