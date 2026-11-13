use crate::particles::materials::{FireParticleMaterial, SmokeParticleMaterial, TractorBeamMaterial, TractorSparkleMaterial};
use crate::particles::materials::SideJetMaterial;
use bevy::app::App;
use bevy::prelude::{Component, Plugin, Update};
use bevy_enoki::EnokiPlugin;
use bevy_enoki::prelude::Particle2dMaterialPlugin;

pub mod materials;
pub mod thrusters;
pub mod tractor_beam;


pub mod particles {
    pub const PARTICLE_TRACTOR_BEAM: &str = "particles/tractor_beam.ron";
    pub const PARTICLE_TRACTOR_BEAM_SPARKLE: &str = "particles/tractor_beam_sparkle.ron";
    pub const PARTICLE_MAIN_THRUSTER: &str = "particles/main_thrust.ron";
    pub const PARTICLE_MAIN_THRUSTER_CORE: &str = "particles/main_thrust_core.ron";
    pub const PARTICLE_RIGHT_THRUSTER: &str = "particles/side_thrust_right.ron";
    pub const PARTICLE_LEFT_THRUSTER: &str = "particles/side_thrust_left.ron";
    pub const PARTICLE_REVERSE_THRUSTER: &str = "particles/reverse_thrust.ron";

    pub const PARTICLE_EXPLOSION_FIRE: &str = "particles/explosion_fire.ron";
    pub const PARTICLE_EXPLOSION_SMOKE: &str = "particles/smoke_large.ron";
    pub const PARTICLE_EXPLOSION_SMOKE_SMALL: &str = "particles/smoke_small.ron";
    pub const PARTICLE_EXPLOSION_SPARKS: &str = "particles/explosion_sparks.ron";

}

mod config {

    pub const TRACTOR_BEAM_Z_OFFSET: f32 = 0.1;
    pub const ENGINE_Z_OFFSET: f32 = 0.3;
}

#[derive(Component)]
pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnokiPlugin)
            .add_plugins(Particle2dMaterialPlugin::<FireParticleMaterial>::default())
            .add_plugins(Particle2dMaterialPlugin::<SmokeParticleMaterial>::default())
            .add_plugins(Particle2dMaterialPlugin::<SideJetMaterial>::default())
            .add_plugins(Particle2dMaterialPlugin::<TractorBeamMaterial>::default())
            .add_plugins(Particle2dMaterialPlugin::<TractorSparkleMaterial>::default())
            .add_observer(thrusters::spawn_emitter)
            .add_observer(tractor_beam::spawn_tractor_beam)
            .add_systems(Update, thrusters::update_emitter);
    }
}
