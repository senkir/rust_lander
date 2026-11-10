use crate::lander::lander::Lander;
use crate::tractor::system::TractorParticle;
use bevy::asset::AssetServer;
use bevy::prelude::{Add, Commands, On, Res, Transform, default};
use bevy_enoki::prelude::ParticleSpawnerState;
use bevy_enoki::{ParticleEffectHandle, ParticleSpawner};
use crate::config;

pub fn spawn_tractor_beam(add: On<Add<Lander>>, mut commands: Commands, server: Res<AssetServer>) {
    //spawn

    let (lander) = add.event().entity;

    commands.entity(lander).with_child((
        TractorParticle,
        //base spawner definition
        ParticleSpawner::default(),
        //specific effect
        ParticleEffectHandle(server.load(super::config::PARTICLE_TRACTOR_BEAM)),
        //not active yet
        ParticleSpawnerState {
            active: false,
            ..default()
        },
        //move it around
        Transform::from_xyz(0.0, 0.0, -super::config::TRACTOR_BEAM_Z_OFFSET),
    ));
}
