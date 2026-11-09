use bevy::app::App;
use bevy::asset::Handle;
use bevy::prelude::{Asset, Component, Image, IntoScheduleConfigs, Plugin, TypePath, Update};
use bevy::render::render_resource::AsBindGroup;
use bevy_enoki::{EnokiPlugin, ParticleEffectHandle};
use bevy_enoki::prelude::{Particle2dMaterial, Particle2dMaterialPlugin};
use crate::particles::materials::FireParticleMaterial;

pub mod particles;
pub mod materials;

#[derive(Component)]
pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EnokiPlugin)
            .add_plugins(Particle2dMaterialPlugin::<FireParticleMaterial>::default())
            .add_systems(Update, (particles::spawn_emitter,
                                  particles::update_emitter,
            ).chain())
        ;
    }

}