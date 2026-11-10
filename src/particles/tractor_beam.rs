use crate::lander::lander::Lander;
use crate::particles::materials::{TractorBeamMaterial, TractorSparkleMaterial};
use crate::beam::system::TractorParticle;
use bevy::asset::{AssetServer, Assets};
use bevy::color::LinearRgba;
use bevy::prelude::{Add, Commands, On, Res, ResMut, Transform, default};
use bevy_enoki::prelude::ParticleSpawnerState;
use bevy_enoki::{ParticleEffectHandle, ParticleSpawner};

pub fn spawn_tractor_beam(
    add: On<Add<Lander>>,
    mut materials: ResMut<Assets<TractorBeamMaterial>>,
    mut sparkle: ResMut<Assets<TractorSparkleMaterial>>,
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    //spawn

    let (lander) = add.event().entity;

    commands.entity(lander).with_child((
        TractorParticle,
        //base spawner definition
        ParticleSpawner(materials.add(
            TractorBeamMaterial { tint: LinearRgba::new(1.0, 1.0, 1.0, 0.5) }   // 50 percent
        )),
        //specific effect
        ParticleEffectHandle(server.load(super::config::PARTICLE_TRACTOR_BEAM)),
        //not active yet
        ParticleSpawnerState {
            active: false,
            ..default()
        },
        //move it around
        Transform::from_xyz(0.0, 0.0, -super::config::TRACTOR_BEAM_Z_OFFSET- 0.05),
    ));

    commands.entity(lander).with_child((
        TractorParticle,
        //base spawner definition
        ParticleSpawner(sparkle.add(TractorSparkleMaterial::default())),
        // ParticleSpawner::default(),
        //specific effect
        ParticleEffectHandle(server.load(super::config::PARTICLE_TRACTOR_BEAM_SPARKLE)),
        //not active yet
        ParticleSpawnerState {
            active: false,
            ..default()
        },
        //move it around
        Transform::from_xyz(0.0, 0.0, -super::config::TRACTOR_BEAM_Z_OFFSET),
    ));
}
