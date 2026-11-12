pub mod beam;
pub mod camera;
pub mod chain_2d;
pub mod config;
pub mod damage;
pub mod deliveries;
pub mod delivery_target;
pub mod lander;
pub mod particles;
pub mod score;
pub mod thruster;
pub mod world;
pub mod state;
pub mod controls;

use crate::beam::TractorPlugin;
use crate::camera::CameraPlugin;
use crate::damage::DamagePlugin;
use crate::deliveries::DeliveriesPlugin;
use crate::lander::LanderPlugin;
use crate::particles::ParticlePlugin;
use crate::score::ScorePlugin;
use crate::world::WorldPlugin;
use avian2d::PhysicsPlugins;
use bevy::DefaultPlugins;
use bevy::camera::ClearColor;
use bevy::color::Color;
use bevy::prelude::{App, ImagePlugin, PluginGroup};
use bevy_ecs_tilemap::TilemapPlugin;
use crate::controls::ControlsPlugin;
use crate::state::StatePlugin;

fn main() {
    // chain_2d::main();
    // collision_layers::main();
    App::new()
        // .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_plugins(StatePlugin)
        .add_plugins(ControlsPlugin)
        .add_plugins(PhysicsPlugins::default().with_length_unit(config::world::WORLD_SIZE))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .add_plugins(WorldPlugin)
        .add_plugins((LanderPlugin, ParticlePlugin))
        .add_plugins(DeliveriesPlugin)
        .add_plugins(DamagePlugin)
        .add_plugins(TractorPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ScorePlugin)
        .run();
    //set up the bevy application screen
    
    //confirm this is correct
    
    DefaultPlugins.set(ImagePlugin::default_nearest());
}
