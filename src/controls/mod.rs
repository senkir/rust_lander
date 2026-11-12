pub mod controls;

use bevy::app::{App, Plugin, Startup};
use bevy::prelude::Component;

#[derive(Component)]
pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, controls::initialize_controls);
    }
}