use crate::damage::health::Health;
use avian2d::prelude::{CollisionStart, Collisions, ContactGraph, Mass, Physics};
use bevy::log::info;
use bevy::prelude::{Component, Entity, Message, MessageReader, MessageWriter, Query, Res, With};
use bevy::time::Time;

use crate::damage;
use crate::deliveries::delivery::Delivery;
use crate::lander::lander::Lander;

const IMPACT_THRESHOLD: f32 = 500.0;

const COLLISION_DAMAGE: u32 = 20;

#[derive(Message)]
pub struct DamageEvent { pub target: Entity, pub amount: u32 }

/// observe any collisions on bodies with health
pub fn observe_collisions(
    mut reader: MessageReader<CollisionStart>,
    collisions: Collisions,
    mut writer: MessageWriter<DamageEvent>,
    mut health: Query<(&Mass, &mut Health), With<Health>>,
    landers: Query<(), With<Lander>>,
    packages: Query<(), With<Delivery>>,
) {

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
            if let Ok((mass, mut health)) = health.get_mut(*body) {
                let impact_speed = impulse / mass.0;
                info!("impact {:?}", impact_speed);

                if (impact_speed > IMPACT_THRESHOLD) {
                    //damage happened

                    writer.write(DamageEvent { target: *body, amount: COLLISION_DAMAGE });

                    if landers.contains(*body) {
                        //lander destruction
                        info!("lander damaged {:?}", body);
                    }
                    if packages.contains(*body) {
                        //package destruction
                        info!("packages damaged {:?}", body);
                    }
                }
            }
        }

    }
}

pub fn apply_damage(
    mut events: MessageReader<DamageEvent>,
    mut health: Query<&mut Health>,
) {
    for event in events.read() {
        if let Ok(mut h) = health.get_mut(event.target) {
            h.apply_damage(event.amount);
        }
    }
}

pub fn render_damage() {

}