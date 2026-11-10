use crate::config;
use crate::lander::input::InputDirection;
use crate::lander::lander::Lander;
use crate::lander::ship::ShipStatus;
use crate::particles::materials::{FireParticleMaterial, SideJetMaterial};
use bevy::log::info;
use bevy::prelude::{
    Add, AssetServer, Assets, Bundle, Changed, Commands, Component, On, Query, Res, ResMut,
    Transform, With, default,
};
use bevy_enoki::prelude::ParticleSpawnerState;
use bevy_enoki::{ParticleEffectHandle, ParticleSpawner};

/// marker component for thruster
#[derive(Component, Copy, Clone, Debug, Eq, PartialEq)]
pub struct ThrustEmitter {
    pub angle: u32,
    pub thrust_type: ThrustType,
}

#[derive(Component, Default, Clone, Copy, Eq, PartialEq, Debug)]
pub enum ThrustType {
    Left,
    Right,
    Reverse,
    #[default]
    Large,
}

impl Default for ThrustEmitter {
    fn default() -> Self {
        Self {
            angle: 0,
            thrust_type: ThrustType::Large,
        }
    }
}

impl ThrustEmitter {
    pub fn new(thrust_type: ThrustType, angle: u32) -> Self {
        Self { thrust_type, angle }
    }
}

fn spawn_emitter_internal(
    server: &AssetServer,
    materials: &mut ResMut<Assets<FireParticleMaterial>>,
    // material: &mut Assets<ColorParticle2dMaterial>,
) -> impl Bundle {
    //replace color particle material with custom Particle2d one
    //need additive blending. ColorParticle2d is the default

    (
        ThrustEmitter::default(),
        //base spawner definition
        ParticleSpawner(materials.add(FireParticleMaterial::default())),
        //specific effect
        ParticleEffectHandle(server.load(super::config::PARTICLE_MAIN_THRUSTER)),
        //not active yet
        ParticleSpawnerState {
            active: false,
            ..default()
        },
        //move it around
        Transform::from_xyz(
            0.0,
            config::thruster::THRUSTER_Y_OFFSET,
            -super::config::ENGINE_Z_OFFSET,
        ),
    )
}
fn spawn_emitter_core_internal(
    server: &AssetServer,
    materials: &mut ResMut<Assets<FireParticleMaterial>>,
    // material: &mut Assets<ColorParticle2dMaterial>,
) -> impl Bundle {
    (
        ThrustEmitter::default(),
        //base spawner definition
        ParticleSpawner(materials.add(FireParticleMaterial::default())),
        //specific effect
        ParticleEffectHandle(server.load(super::config::PARTICLE_MAIN_THRUSTER_CORE)),
        //not active yet
        ParticleSpawnerState {
            active: false,
            ..default()
        },
        //move it around
        Transform::from_xyz(
            0.0,
            config::thruster::THRUSTER_Y_OFFSET,
            -super::config::ENGINE_Z_OFFSET + 0.05, //slight offset for core
        ),
    )
}
fn spawn_emitter_small_internal(
    server: &AssetServer,
    thrust_type: ThrustType,
    materials: &mut ResMut<Assets<SideJetMaterial>>,
) -> impl Bundle {
    //replace color particle material with custom Particle2d one
    //need additive blending. ColorParticle2d is the default

    (
        ThrustEmitter::new(thrust_type, 90),
        //base spawner definition
        ParticleSpawner(materials.add(SideJetMaterial::default())),
        //specific effect
        ParticleEffectHandle(if (thrust_type == ThrustType::Right) {
            server.load(super::config::PARTICLE_LEFT_THRUSTER)
        } else if (thrust_type == ThrustType::Left) {
            server.load(super::config::PARTICLE_RIGHT_THRUSTER)
        } else if (thrust_type == ThrustType::Reverse) {
            server.load(super::config::PARTICLE_REVERSE_THRUSTER)
        } else {
            panic!("ThrustEmitter does not support thrust type")
        }),
        //not active yet
        ParticleSpawnerState {
            active: false,
            ..default()
        },
        //move it around
        Transform::from_xyz(0.0, 0.0, -super::config::ENGINE_Z_OFFSET),
    )
}

pub fn spawn_emitter(
    add: On<Add<Lander>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<FireParticleMaterial>>,
    mut side_material: ResMut<Assets<SideJetMaterial>>,
    server: Res<AssetServer>,
) {
    // let mat = materials.add(ColorParticle2dMaterial::new(
    //     server.load("flame.png"),
    //     1, 1, // sprite sheet columns, rows (1,1 for a single image)
    // ));
    //

    let (lander) = add.event().entity;
    //TODO: only run this once
    // if (ship_status.is_thrusting) {
    info!("thrusting spawned! {}", lander);
    //Spawn the child

    //main thruster
    commands
        .entity(lander)
        .with_child(spawn_emitter_internal(&server, &mut materials));

    commands
        .entity(lander)
        .with_child(spawn_emitter_core_internal(&server, &mut materials));

    commands
        .entity(lander)
        .with_child(spawn_emitter_small_internal(
            &server,
            ThrustType::Left,
            &mut side_material,
        ));

    commands
        .entity(lander)
        .with_child(spawn_emitter_small_internal(
            &server,
            ThrustType::Right,
            &mut side_material,
        ));

    commands
        .entity(lander)
        .with_child(spawn_emitter_small_internal(
            &server,
            ThrustType::Reverse,
            &mut side_material,
        ));
}

pub fn update_emitter(
    ship: Query<&ShipStatus, (With<Lander>, Changed<ShipStatus>)>,
    mut emitters: Query<
        (&mut ParticleSpawnerState, &ThrustEmitter, &mut Transform),
        (With<ThrustEmitter>),
    >,
) {
    let Ok(ship_status) = ship.single() else {
        //returns Err when there are no matches (like the filter)
        return;
    };

    for (mut spawner_state, thrust_emitter, mut transform) in emitters.iter_mut() {
        //toggle the large thruster
        match (thrust_emitter.thrust_type) {
            ThrustType::Large => {
                if (!spawner_state.active && ship_status.thrust_direction == InputDirection::Forward) {
                    spawner_state.active = true;
                } else {
                    spawner_state.active = false;
                }
            }
            ThrustType::Left => {
                if (!spawner_state.active && ship_status.thrust_direction == InputDirection::Left) {
                    spawner_state.active = true;
                } else {
                    spawner_state.active = false;
                }
            }
            ThrustType::Right => {
                if (!spawner_state.active && ship_status.thrust_direction == InputDirection::Right)
                {
                    spawner_state.active = true;
                } else {
                    spawner_state.active = false;
                }
            }
            ThrustType::Reverse => {
                if (!spawner_state.active
                    && ship_status.thrust_direction == InputDirection::Backward)
                {
                    spawner_state.active = true;
                } else {
                    spawner_state.active = false;
                }
            }
        }

        // //modify the small thruster
        // if (thrust_emitter.thrust_type == ThrustType::Left) {
        //     if (spawner_state.active != ship_status.use_secondary_thrust()
        //         && ship_status.thrust_direction == InputDirection::Left)
        //     {
        //         spawner_state.active = ship_status.use_secondary_thrust();
        //     } else {
        //         spawner_state.active = false;
        //     }
        // }

        // if (thrust_emitter.thrust_type == ThrustType::Right) {
        //     if (spawner_state.active != ship_status.use_secondary_thrust()
        //         && ship_status.thrust_direction == InputDirection::Right)
        //     {
        //         spawner_state.active = ship_status.use_secondary_thrust();
        //     } else {
        //         spawner_state.active = false;
        //     }
        // }
    }
}
