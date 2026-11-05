use bevy::app::{App, Startup};
use bevy::prelude::{Component, Plugin};

pub mod lander;
pub mod physics;
pub mod spawn;

#[derive(Component)]
pub struct LanderPlugin;

impl Plugin for LanderPlugin {
    fn build(&self, app: &mut App) {
        //TODO: create systems here
        app.add_systems(Startup, spawn::spawn_lander);
    }
}