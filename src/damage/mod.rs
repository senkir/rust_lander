use crate::damage::collision::DamageEvent;
pub(crate) use crate::hud::health_bar;
use crate::lander;
use crate::state::GameState;
use bevy::app::{App, Plugin, Update};
use bevy::math::ops::exp;
use bevy::prelude::{Component, IntoScheduleConfigs, in_state};
use crate::damage::explosion::DeathTrigger;

pub mod collision;
pub mod health;
pub mod notification;
pub mod explosion;
pub mod death;
pub mod damage_stage;
pub mod smoke;

#[derive(Component)]
pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DamageEvent>()
            .add_message::<DeathTrigger>()
            .add_systems(
            Update,
            (
                collision::observe_collisions,
                notification::spawn_damage_number,
                notification::animate_floating_text,
                health_bar::update_lander_health,
                smoke::observe_terrain_collisions,
                death::observe_health.run_if(in_state(GameState::Playing)),
                death::countdown_death_timer.run_if(in_state(GameState::Playing)),
                lander::spawn::update_lander_frame,
                explosion::spawn_explosion.run_if(in_state(GameState::Playing)),
            )
                .chain(),
        );
    }
}
