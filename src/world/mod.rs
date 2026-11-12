use avian2d::prelude::Gravity;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::math::Vec2;
use bevy::prelude::{Component, IntoScheduleConfigs, SystemSet};

pub mod layers;
pub mod spawn;
pub mod terrain;

use config::world;

use crate::config;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TerrainReady;

#[derive(Component)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec2::new(0.0, -world::GRAVITY)))
            .add_systems(Update, terrain::draw_terrain);
    }
}
