use crate::lander::lander::Lander;
use crate::tractor::ship::{ShipTractor, TractorState};
use bevy::log::{error, info};
use bevy::prelude::{ButtonInput, Changed, KeyCode, Query, Res, With};

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

/// detect when something is changing about the ship tractor beam
pub fn handle_state_change(mut query: Query<&mut ShipTractor, Changed<ShipTractor>>) {
    error!("handle_state_change called");
    let Ok(mut ship_tractor) = query.single() else {
        return;
    };

    if (ship_tractor.state == TractorState::TurningOn) {
        info!("tractor on");
        //spawn
    }
    if (ship_tractor.state == TractorState::TurningOff) {
        info!("tractor off");
        //despawn
    }
}

pub fn spawn_tractor_beam(mut query: Query<&mut ShipTractor, Changed<ShipTractor>>) {
    let Ok(mut ship_tractor) = query.single_mut() else {
        return;
    };

    if (ship_tractor.state == TractorState::TurningOn) {
        //spawn
        info!("tractor on");
        ship_tractor.state = TractorState::On;
    }
}

pub fn despawn_tractor_beam(mut query: Query<&mut ShipTractor, Changed<ShipTractor>>) {
    let Ok(mut ship_tractor) = query.single_mut() else {
        return;
    };

    if (ship_tractor.state == TractorState::TurningOff) {
        //spawn
        info!("tractor off");
        ship_tractor.state = TractorState::Off;
    }
}
