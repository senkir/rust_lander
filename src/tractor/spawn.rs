use crate::lander::lander::Lander;
use crate::tractor::ship::{ShipTractor, TractorState};
use bevy::asset::AssetServer;
use bevy::log::{error, info};
use bevy::prelude::{default, Added, ButtonInput, Changed, Commands, Component, Entity, KeyCode, Query, Res, Transform, With};
use bevy_enoki::prelude::ParticleSpawnerState;
use bevy_enoki::{ParticleEffectHandle, ParticleSpawner};

#[derive(Component)]
pub struct TractorParticle;

/// determine when the key is being used for tractor beam function
pub fn observe_tractor_beam_start(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut ShipTractor, With<Lander>>,
) {
    if (input.just_pressed(KeyCode::Space)) {
        let Ok(mut tractor_state) = query.single_mut() else {
            return;
        };
        info!("tractor turning on");

        tractor_state.state = TractorState::TurningOn;
    }

    if (input.just_released(KeyCode::Space)) {
        let Ok(mut tractor_state) = query.single_mut() else {
            return;
        };
        info!("tractor turning off");

        tractor_state.state = TractorState::TurningOff;
    }
}

pub fn spawn_tractor_beam(
    mut commands: Commands,
    server: Res<AssetServer>,
    lander_query: Query<Entity, Added<Lander>>,
) {
    //spawn


    for (lander) in lander_query.iter() {
        error!("spawning tractor particle gen");
        commands.entity(lander).with_child((
            TractorParticle,
            //base spawner definition
            ParticleSpawner::default(),
            //specific effect
            ParticleEffectHandle(server.load("tractor_ui.ron")),
            //not active yet
            ParticleSpawnerState {
                active: false,
                ..default()
            },
            //move it around
            Transform::from_xyz(0.0, 0.0, 0.1),
        ));
        //just end it here
        return;
    }
}

pub fn enable_tractor_beam(
    mut lander_query: Query<&mut ParticleSpawnerState, With<TractorParticle>>,
    mut query: Query<&mut ShipTractor, Changed<ShipTractor>>,
) {
    let Ok(mut ship_tractor) = query.single_mut() else {
        return;
    };

    if (ship_tractor.state == TractorState::TurningOn) {
        //spawn
        ship_tractor.state = TractorState::On;

        let Ok(mut spawner_state) = lander_query.single_mut() else {
            error!("lander spawner state not set");
            return;
        };
        info!("tractor on");
        spawner_state.active = true;
    }
}

pub fn disable_tractor_beam(
    mut lander_query: Query<&mut ParticleSpawnerState, With<TractorParticle>>,
    mut query: Query<&mut ShipTractor, Changed<ShipTractor>>,
) {
    let Ok(mut ship_tractor) = query.single_mut() else {
        return;
    };

    if ship_tractor.state == TractorState::TurningOff {
        ship_tractor.state = TractorState::Off;

        let Ok(mut spawner_state) = lander_query.single_mut() else {
            return;
        };
        info!("tractor off");
        spawner_state.active = false;
    }
}