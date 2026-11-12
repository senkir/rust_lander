use crate::damage::collision::DamageEvent;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::{in_state, Component, IntoScheduleConfigs};
use crate::lander;
use crate::state::GameState;

pub mod collision;
pub mod health;
pub mod notification;
pub mod bar;
pub mod explosion;
pub mod death;

#[derive(Component)]
pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DamageEvent>()
            .add_systems(
            Update,
            (
                collision::observe_collisions,
                notification::spawn_damage_number,
                notification::animate_floating_text,
                bar::update_lander_health,
                death::observe_health.run_if(in_state(GameState::Playing)),
                lander::spawn::update_lander_frame
            )
                .chain(),
        );
    }
}
