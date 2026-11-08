use bevy::app::{App, Plugin, Update};
use bevy::prelude::Component;

pub mod health;
pub mod collision;
#[derive(Component)]
pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision::observe_collisions);
    }
}