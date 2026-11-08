use crate::damage::health::Health;
use avian2d::prelude::{CollisionStart, Collisions, ContactGraph, Mass, Physics};
use bevy::log::info;
use bevy::prelude::{Entity, MessageReader, Query, Res, With};
use bevy::time::Time;

use crate::damage;

/// observe any collisions on bodies with health
pub fn observe_collisions(
    mut reader: MessageReader<CollisionStart>,
    query: Query<Entity, With<Health>>,
    collisions: Collisions,
    health: Query<&Mass, With<Health>>,
) {
    let mut total_impulse = 0.0;

    for collision_event in reader.read() {
        info!("Collision event: {:?}", collision_event);

        let Some(pair) = collisions.get(collision_event.collider1, collision_event.collider2)
        else {
            continue;
        };

        let (Some(body_a), Some(body_b)) =
            (collision_event.body1, collision_event.body2) else {
            continue;
        };

        let impulse = pair.total_normal_impulse_magnitude();

        info!("impulse at collision {:?}", impulse);

        //try to take into account impact to generate a result
        for body in [body_a, body_b].iter() {
            if let Ok(mass) = health.get(*body) {
                let impact_speed = impulse / mass.0;
                info!("impact {:?}", impact_speed);
            }
        }

    }
}
