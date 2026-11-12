use crate::damage::health::Health;
use crate::lander::lander::Lander;
use crate::state::GameState;
use bevy::log::info;
use bevy::prelude::{NextState, Query, ResMut, With};

pub fn observe_health(
    query: Query<&Health, With<Lander>>,
    mut state: ResMut<NextState<GameState>>,
) {
    for health in query.iter() {
        if (health.current == 0) {
            info!("marking for death? Health: {:?}", health);
            //mark for death
            state.set(GameState::Dead)
        }
    }
}
