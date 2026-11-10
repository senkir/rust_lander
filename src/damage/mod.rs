use crate::damage::collision::DamageEvent;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Component, IntoScheduleConfigs};

pub mod collision;
pub mod health;
pub mod notification;

#[derive(Component)]
pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DamageEvent>().add_systems(
            Update,
            (
                collision::observe_collisions,
                collision::apply_damage,
                notification::spawn_damage_number,
                notification::animate_floating_text,
            )
                .chain(),
        );
    }
}
