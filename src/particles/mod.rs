use bevy::app::App;
use bevy::asset::Handle;
use bevy::prelude::{Asset, Component, Image, IntoScheduleConfigs, Plugin, TypePath, Update};
use bevy::render::render_resource::AsBindGroup;
use bevy_enoki::{EnokiPlugin, ParticleEffectHandle};
use bevy_enoki::prelude::{Particle2dMaterial, Particle2dMaterialPlugin};

pub mod particles;


#[derive(Component)]
pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EnokiPlugin)
            .add_systems(Update, (particles::spawn_emitter, particles::update_emitter).chain())
        ;
    }

}

#[derive(AsBindGroup, Asset, TypePath, Clone, Default)]
pub struct FireParticleMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Handle<Image>,
}
impl Particle2dMaterial for FireParticleMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "sample_mat.wesl".into()
    }
}
