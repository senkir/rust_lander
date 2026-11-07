use crate::config;
use crate::lander::lander::Lander;
use crate::lander::physics::ShipStatus;
use crate::thrusters::components::ParticleEmitter;
use avian2d::prelude::Forces;
use bevy::log::{error, info};
use bevy::prelude::{
    Added, Asset, AssetServer, Children, Commands, Component, Entity, GetField, LinearRgba, Query,
    Ref, Res, Transform, TypePath, Vec2, Vec3, With, Without, default,
};
use bevy::time::Time;
use bevy_enoki::prelude::{ColorParticle2dMaterial, MultiCurve, ParticleSpawnerState, Rval};
use bevy_enoki::{ParticleEffectHandle, ParticleSpawner};
use serde::Deserialize;
use std::thread::spawn;

/// marker component for thruster
#[derive(Component)]
pub struct ThrustEmitter;

pub fn spawn_emitter(
    mut commands: Commands,
    server: Res<AssetServer>,
    query: Query<(Entity), (Added<Lander>)>, // runs once per lander
) {
    // bring in your own effect asset from a ron file
    // (hot reload by default)
    //TODO: write logic for a custom particle ron file

    let Ok(lander) = query.single() else {
        return;
    };
    //
    // //we only want the negative case
    // let Err(_) = emitter.single() else {
    //     error!("thruster already exists!");
    //     return;
    // };

    //TODO: only run this once
    // if (ship_status.is_thrusting) {
    info!("thrusting spawned!");
    //Spawn the child
    commands.entity(lander).with_child((
        ThrustEmitter,
        //base spawner definition
        ParticleSpawner::default(),
        //specific effect
        ParticleEffectHandle(server.load("spawner.ron")),
        //not active yet
        ParticleSpawnerState {
            active: false,
            ..default()
        },
        //move it around
        Transform::from_xyz(0.0, -config::lander::LANDER_SIZE / 2.0, 0.1),
    ));
    // }
}

pub fn update_emitter(
    ship: Query<&ShipStatus, (With<Lander>)>,
    mut emitters: Query<&mut ParticleSpawnerState, (With<ThrustEmitter>)>,
) {
    let Ok((mut spawner_state)) = emitters.single_mut() else {
        return;
    };

    let Ok(ship_status) = ship.single() else {
        return;
    };

    spawner_state.active = ship_status.is_thrusting;
}

pub fn despawn_emitter(
    mut commands: Commands,
    ship: Query<&ShipStatus, (With<Lander>)>,
    query: Query<Entity, (With<ThrustEmitter>)>,
) {
    let Ok(thrust_emitter) = query.single() else {
        return;
    };

    let Ok(ship_status) = ship.single() else {
        return;
    };
    if (!ship_status.is_thrusting) {
        info!("thrusting stopped!");
        commands.entity(thrust_emitter).despawn();
    }
}
