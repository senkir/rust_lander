use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Component, IntoScheduleConfigs};

pub mod attraction;
pub mod ship;
pub mod system;

const HOLD_OFFSET: f32 = 45.0;      // lander half-size + package half-size + margin

const BEAM_RANGE: f32 = 150.0;
const BEAM_STRENGTH: f32 = 20_000.0; // tune against Mass and Gravity
const BEAM_DAMPING: f32 = 300.0;

const BEAM_STIFFNESS: f32 = 80.0;


#[derive(Component)]
pub struct TractorPlugin;

impl Plugin for TractorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                system::observe_tractor_beam_start,
                system::enable_tractor_beam,
                system::disable_tractor_beam,
                attraction::apply_tractor_force,
            )
                .chain(),
        );
    }
}
