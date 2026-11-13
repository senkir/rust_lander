pub mod beam;
pub mod camera;
pub mod chain_2d;
pub mod config;
pub mod damage;
pub mod deliveries;
pub mod lander;
pub mod particles;
pub mod thruster;
pub mod world;
pub mod state;
pub mod controls;
pub mod hud;
pub mod theme;
pub mod anim;
use wasm_bindgen::prelude::*;

use crate::beam::TractorPlugin;
use crate::camera::CameraPlugin;
use crate::controls::ControlsPlugin;
use crate::damage::DamagePlugin;
use crate::deliveries::DeliveriesPlugin;
use crate::lander::LanderPlugin;
use crate::particles::ParticlePlugin;
use crate::state::StatePlugin;
use crate::world::WorldPlugin;
use avian2d::PhysicsPlugins;
use bevy::audio::AudioPlugin;
use bevy::DefaultPlugins;
use bevy::camera::ClearColor;
use bevy::color::Color;
use bevy::prelude::{App, ImagePlugin, PluginGroup};
use bevy_ecs_tilemap::TilemapPlugin;
use hud::score::ScorePlugin;
use crate::anim::AutoAnimatorPlugin;
use crate::theme::ThemePlugin;

#[wasm_bindgen(start)]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AudioPlugin::default())
        .add_plugins(TilemapPlugin)
        .add_plugins(StatePlugin)
        .add_plugins(AutoAnimatorPlugin)
        .add_plugins(ControlsPlugin)
        .add_plugins(ThemePlugin)
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
