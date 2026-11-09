use crate::config;
use crate::lander::lander::Lander;
use crate::lander::ship::ShipStatus;
use bevy::log::info;
use bevy::prelude::{default, Added, AssetServer, Bundle, Commands, Component, Entity, Query, Res, Transform, With, Assets, ResMut};
use bevy_enoki::prelude::{ColorParticle2dMaterial, ParticleSpawnerState};
use bevy_enoki::{ParticleEffectHandle, ParticleSpawner};
use crate::particles::materials::FireParticleMaterial;

/// marker component for thruster
#[derive(Component)]
pub struct ThrustEmitter {
    pub angle: u32,
    pub thrust_type: ThrustType
}

#[derive(Component, Default, Clone, Copy, Eq, PartialEq, Debug)]
pub enum ThrustType {
    Small,
    #[default]
    Large
}

impl Default for ThrustEmitter {
    fn default() -> Self {
        Self {
            angle: 0,
            thrust_type: ThrustType::Large
        }
    }
}

fn spawn_emitter_internal(
    server: &AssetServer,
    mut materials: &mut ResMut<Assets<FireParticleMaterial>>,
    // material: &mut Assets<ColorParticle2dMaterial>,
) -> impl Bundle {
    //replace color particle material with custom Particle2d one
    //need additive blending. ColorParticle2d is the default

    (
        ThrustEmitter::default(),
        //base spawner definition
        // ParticleSpawner::from(material),
        ParticleSpawner(materials.add(FireParticleMaterial::default())),
        //specific effect
        ParticleEffectHandle(server.load("main_thrust.ron")),
        //not active yet
        ParticleSpawnerState {
            active: false,
            ..default()
        },
        //move it around
        Transform::from_xyz(0.0, config::thruster::THRUSTER_Y_OFFSET, 0.1),
    )
}

pub fn spawn_emitter(
    mut commands: Commands,
    mut materials: ResMut<Assets<FireParticleMaterial>>,
    server: Res<AssetServer>,
    entity: Query<Entity, Added<Lander>>, // runs once per lander
) {
    // bring in your own effect asset from a ron file
    // (hot reload by default)
    //TODO: write logic for a custom particle ron file

    // let mat = materials.add(ColorParticle2dMaterial::new(
    //     server.load("flame.png"),
    //     1, 1, // sprite sheet columns, rows (1,1 for a single image)
    // ));
    //

    // let flame_material = FireParticleMaterial::default();

    for (lander) in entity.iter() {

        //TODO: only run this once
        // if (ship_status.is_thrusting) {
        info!("thrusting spawned! {}", lander);
        //Spawn the child
        commands.entity(lander).with_child(spawn_emitter_internal(&server, &mut materials));
        // commands.entity(lander).insert((
        //     ThrustEmitter::default(),
        //     //base spawner definition
        //     ParticleSpawner::default(),
        //     //specific effect
        //     ParticleEffectHandle(server.load("main_thrust.ron")),
        //     //not active yet
        //     ParticleSpawnerState {
        //         active: false,
        //         ..default()
        //     },
        //     //move it around
        //     Transform::from_xyz(0.0, config::thruster::THRUSTER_Y_OFFSET, 0.1),
        // ));

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
