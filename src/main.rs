pub mod chain_2d;
pub mod config;
pub mod damage;
pub mod deliveries;
pub mod lander;
pub mod particles;
pub mod thruster;
pub mod tractor;
pub mod world;
pub mod camera;

use crate::damage::DamagePlugin;
use crate::deliveries::DeliveriesPlugin;
use crate::lander::LanderPlugin;
use crate::particles::ParticlePlugin;
use crate::tractor::TractorPlugin;
use crate::world::WorldPlugin;
use avian2d::PhysicsPlugins;
use bevy::DefaultPlugins;
use bevy::app::Startup;
use bevy::camera::{Camera, Camera2d, ClearColor, ClearColorConfig};
use bevy::color::Color;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::{App, Commands};
use bevy::render::view::DebandDither;
use bevy::utils::default;
use crate::camera::CameraPlugin;

fn main() {
    // chain_2d::main();
    // collision_layers::main();
    App::new()
        // .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default().with_length_unit(config::world::WORLD_SIZE))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .add_plugins(WorldPlugin)
        .add_plugins((LanderPlugin, ParticlePlugin))
        .add_plugins(DeliveriesPlugin)
        .add_plugins(DamagePlugin)
        .add_plugins(TractorPlugin)
        .add_plugins(CameraPlugin)
        .run();
    //set up the bevy application screen
}
