use avian2d::prelude::Gravity;
use bevy::app::{App, Plugin, Startup};
use bevy::math::Vec2;
use bevy::prelude::Component;

pub mod layers;
pub mod spawn;

#[derive(Component)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            Gravity(Vec2::new(0.0, -9.8))
        )
            .add_systems(Startup, spawn::spawn_world);
    }
}