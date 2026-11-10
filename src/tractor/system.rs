use crate::lander::lander::Lander;
use crate::tractor::ship::{ShipTractor, TractorState};
use bevy::log::info;
use bevy::prelude::{ButtonInput, Changed, Component, KeyCode, Query, Res, With};
use bevy_enoki::prelude::ParticleSpawnerState;

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

pub fn enable_tractor_beam(
    mut particle_query: Query<&mut ParticleSpawnerState, With<TractorParticle>>,
    mut ship_query: Query<&mut ShipTractor, Changed<ShipTractor>>,
) {
    let Ok(mut ship_tractor) = ship_query.single_mut() else {
        return;
    };

    if (ship_tractor.state == TractorState::TurningOn) {
        //spawn
        ship_tractor.state = TractorState::On;

        for (mut spawner_state) in particle_query.iter_mut() {
            info!("tractor on");
            spawner_state.active = true;
        }
    }
}

pub fn disable_tractor_beam(
    mut particle_query: Query<&mut ParticleSpawnerState, With<TractorParticle>>,
    mut ship_query: Query<&mut ShipTractor, Changed<ShipTractor>>,
) {
    let Ok(mut ship_tractor) = ship_query.single_mut() else {
        return;
    };

    if ship_tractor.state == TractorState::TurningOff {
        ship_tractor.state = TractorState::Off;

        for (mut spawner_state) in particle_query.iter_mut() {
            info!("tractor off");
            spawner_state.active = false;
        }
    }
}
