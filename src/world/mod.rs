use avian2d::prelude::Gravity;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::math::Vec2;
use bevy::prelude::{Component, IntoScheduleConfigs};

pub mod layers;
pub mod spawn;
pub mod terrain;

use config::world;

use crate::config;

#[derive(Component)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec2::new(0.0, -world::GRAVITY)))
            .add_systems(
                Startup,
                (
                    spawn::spawn_world,
                    spawn::generate_bounds,
                    terrain::spawn_terrain,
                )
                    .chain(),
            )
            .add_systems(Update, terrain::draw_terrain);
    }
}
