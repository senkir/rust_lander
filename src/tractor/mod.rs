use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::{Component, IntoScheduleConfigs};

pub mod attraction;
pub mod ship;
pub mod spawn;

const BEAM_RANGE: f32 = 150.0;
const BEAM_STRENGTH: f32 = 300_000.0; // tune against Mass and Gravity
const BEAM_DAMPING: f32 = 8.0;

#[derive(Component)]
pub struct TractorPlugin;

impl Plugin for TractorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn::spawn_tractor_beam,
                spawn::observe_tractor_beam_start,
                spawn::enable_tractor_beam,
                spawn::disable_tractor_beam,
                attraction::apply_tractor_force,
            )
                .chain(),
        );
    }
}
