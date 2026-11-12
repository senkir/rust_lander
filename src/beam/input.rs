use crate::lander::lander::Lander;
use crate::beam::ship::{ShipTractor, TractorState};
use bevy::log::info;
use bevy::prelude::{ButtonInput, Changed, Commands, Component, Deref, DerefMut, KeyCode, Query, Res, ResMut, Resource, Time, With};
use bevy::time::{Timer, TimerMode};
use bevy_enoki::prelude::ParticleSpawnerState;
use crate::controls::controls::ControlsResource;

#[derive(Component)]
pub struct TractorParticle;

const TRACTOR_BEAM_DURATION_S: f32 = 3.0;
const TRACTOR_BEAM_COOLDOWN_S: f32 = 2.0;


// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct BeamTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
struct CooldownTimer(Timer);

// Tick the timer, and change state when finished
fn countdown(
    time: Res<Time>,
    mut timer: ResMut<BeamTimer>,
) {
    if timer.tick(time.delta()).is_finished() {
        //end the beam

    }
}

/// determine when the key is being used for beam beam function
pub fn observe_tractor_beam_start(
    mut commands: Commands,
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

        if turning_on {
            commands.insert_resource(
                BeamTimer(
                Timer::from_seconds(TRACTOR_BEAM_DURATION_S, TimerMode::Once)));
        }   

    }

}
