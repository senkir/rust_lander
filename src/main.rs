pub mod lander;
pub mod world;
pub mod chain_2d;
pub mod collision_layers;
pub mod config;

use crate::lander::LanderPlugin;
use crate::world::WorldPlugin;
use avian2d::PhysicsPlugins;
use bevy::DefaultPlugins;
use bevy::app::Startup;
use bevy::camera::{Camera2d, ClearColor};
use bevy::color::Color;
use bevy::prelude::{App, Commands};

fn main() {
    // chain_2d::main();
    // collision_layers::main();
    App::new()
        // .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default().with_length_unit(config::world::WORLD_SIZE))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .add_plugins(WorldPlugin)
        .add_plugins(LanderPlugin)
        .add_systems(Startup, create_camera)
        .run();
    //set up the bevy application screen
}

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
