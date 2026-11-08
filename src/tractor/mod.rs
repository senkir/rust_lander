use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Component, IntoScheduleConfigs};

pub mod attraction;
pub mod spawn;
pub mod ship;

const BEAM_RANGE: f32 = 400.0;
const BEAM_STRENGTH: f32 = 400_000.0; // tune against Mass and Gravity
const BEAM_DAMPING: f32 = 8.0;

#[derive(Component)]
pub struct TractorPlugin;

impl Plugin for TractorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn::observe_tractor_beam_start,
                spawn::spawn_tractor_beam,
                spawn::despawn_tractor_beam,
                attraction::apply_tractor_force,
            ).chain(),
        );
    }
}
