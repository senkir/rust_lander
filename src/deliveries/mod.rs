use bevy::app::{App, Startup};
use bevy::prelude::{Component, Plugin};

pub mod delivery;
pub mod spawn;

#[derive(Component)]
pub struct DeliveriesPlugin;

impl Plugin for DeliveriesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn::spawn_package);
    }
}