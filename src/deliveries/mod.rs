use bevy::app::{App, Startup};
use bevy::prelude::{Component, IntoScheduleConfigs, Plugin};
use crate::world::TerrainReady;

pub mod delivery;
pub mod spawn;

#[derive(Component)]
pub struct DeliveriesPlugin;

impl Plugin for DeliveriesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn::spawn_package)
            .add_systems(Startup, 
                         crate::delivery_target::spawn::spawn_target.after(TerrainReady))
        ;
    }
}


mod config {

    pub const MASS: f32 = 50.0;
}