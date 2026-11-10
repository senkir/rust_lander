use bevy::app::{App, Startup};
use bevy::prelude::Plugin;

pub mod score;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, score::generate_score);
    }
}