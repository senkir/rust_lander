use crate::config;
use crate::lander::lander::Lander;
use crate::lander::ship::ShipStatus;
use bevy::log::info;
use bevy::prelude::{
    Added, AssetServer, Commands, Component, Entity, Query, Res, Transform, With, default,
};
use bevy_enoki::prelude::ParticleSpawnerState;
use bevy_enoki::{ParticleEffectHandle, ParticleSpawner};

/// marker component for thruster
#[derive(Component)]
pub struct ThrustEmitter;

pub fn spawn_emitter(
    mut commands: Commands,
    server: Res<AssetServer>,
    entity: Query<Entity, Added<Lander>>, // runs once per lander
) {
    // bring in your own effect asset from a ron file
    // (hot reload by default)
    //TODO: write logic for a custom particle ron file

    for (lander) in entity.iter() {
        //
        // //we only want the negative case
        // let Err(_) = emitter.single() else {
        //     error!("thruster already exists!");
        //     return;
        // };

        //TODO: only run this once
        // if (ship_status.is_thrusting) {
        info!("thrusting spawned! {}", lander);
        //Spawn the child
        commands.entity(lander).with_child((
            ThrustEmitter,
            //base spawner definition
            ParticleSpawner::default(),
            //specific effect
            ParticleEffectHandle(server.load("main_thrust.ron")),
            //not active yet
            ParticleSpawnerState {
                active: false,
                ..default()
            },
            //move it around
            Transform::from_xyz(0.0, config::thruster::THRUSTER_Y_OFFSET, 0.1),
        ));

        //left
        // commands.entity(lander).with_child((
        //     ThrustEmitter,
        //     //base spawner definition
        //     ParticleSpawner::default(),
        //     //specific effect
        //     ParticleEffectHandle(server.load("side_thrust.ron")),
        //     //not active yet
        //     ParticleSpawnerState {
        //         active: false,
        //         ..default()
        //     },
        //     //move it around ,
        //     Transform::from_xyz(-config::thruster::SIDE_THRUSTER_X_OFFSET,
        //                         config::thruster::SIDE_THRUSTER_Y_OFFSET, 0.1),
        // ));
        // right
        // commands.entity(lander).with_child((
        //     ThrustEmitter,
        //     //base spawner definition
        //     ParticleSpawner::default(),
        //     //specific effect
        //     ParticleEffectHandle(server.load("side_thrust.ron")),
        //     //not active yet
        //     ParticleSpawnerState {
        //         active: false,
        //         ..default()
        //     },
        //     //move it around
        //     Transform::from_xyz(config::thruster::SIDE_THRUSTER_X_OFFSET,
        //                         config::thruster::SIDE_THRUSTER_Y_OFFSET, 0.1),
        // ));
        // }
    }
}

pub fn update_emitter(
    ship: Query<&ShipStatus, (With<Lander>)>,
    mut emitters: Query<&mut ParticleSpawnerState, (With<ThrustEmitter>)>,
) {
    let Ok(ship_status) = ship.single() else {
        return;
    };

    let Ok((mut spawner_state)) = emitters.single_mut() else {
        return;
    };

    // if ship_status.is_thrusting {
    //     info!("thrust on!! ship_status = {:?}", ship_status);
    // }

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
