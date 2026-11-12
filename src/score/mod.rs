use crate::score::display::ScoreUpdate;
use crate::score::score::Score;
use bevy::app::{App, Startup};
use bevy::prelude::{Plugin, Update};

pub mod display;
pub mod score;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
        app.add_message::<ScoreUpdate>()
            .add_systems(Update, display::update_score);
    }
}
