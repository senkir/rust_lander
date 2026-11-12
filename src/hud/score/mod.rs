use crate::hud::score::display::ScoreUpdate;
use crate::hud::score::score::Score;
use bevy::app::App;
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
