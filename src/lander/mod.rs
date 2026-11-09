use crate::thruster::engine_thruster;
use bevy::app::{App, Startup};
use bevy::prelude::{Component, Plugin, Update};

pub mod lander;
pub mod spawn;
pub mod rotation_thruster;
pub mod debug;
pub mod ship;
pub mod input;

#[derive(Component)]
pub struct LanderPlugin;

impl Plugin for LanderPlugin {
    fn build(&self, app: &mut App) {
        //TODO: create systems here
        app.add_systems(Startup, spawn::spawn_lander)
            .add_systems(Update, (
                input::observe_thruster_pressed,
                engine_thruster::handle_thrust_active_state,
                rotation_thruster::handle_rotation_thrust,
                debug::debug_reset_sprite,
                debug::debug_show_thrust
                ));
    }
}