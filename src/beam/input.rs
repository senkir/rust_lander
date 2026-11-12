use crate::lander::lander::Lander;
use crate::beam::ship::{ShipTractor, TractorState};
use bevy::log::info;
use bevy::prelude::{ButtonInput, Changed, Component, KeyCode, Query, Res, With};
use bevy_enoki::prelude::ParticleSpawnerState;
use crate::controls::controls::ControlsResource;

#[derive(Component)]
pub struct TractorParticle;

/// determine when the key is being used for beam beam function
pub fn observe_tractor_beam_start(
    input: Res<ButtonInput<KeyCode>>,
    controls: Res<ControlsResource>,
    mut particle_query: Query<&mut ParticleSpawnerState, With<TractorParticle>>,
    mut query: Query<&mut ShipTractor, With<Lander>>,
) {
    if (input.just_pressed(controls.tractor_beam)) {
        let Ok(mut tractor_state) = query.single_mut() else {
            return;
        };
        info!("beam turning on");

        let turning_on = if (tractor_state.state == TractorState::On) {
                tractor_state.state = TractorState::Off;
                false
            } else if (tractor_state.state == TractorState::Off) {
                tractor_state.state = TractorState::On;
                true
            } else {
            false
        };

        for (mut spawner_state) in particle_query.iter_mut() {
            spawner_state.active = turning_on;
        }

    }

}
