use crate::damage::health::Health;
use crate::deliveries::delivery::Delivery;
use crate::lander::lander::Lander;
use avian2d::prelude::{CollisionStart, Collisions, ContactPair, Mass};
use bevy::log::info;
use bevy::prelude::{Entity, Message, MessageReader, MessageWriter, Query, With};

const IMPACT_THRESHOLD: f32 = 500.0;

const COLLISION_DAMAGE: u32 = 20;

#[derive(Message)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: u32,
}

#[derive(Message)]
pub struct LanderDamageTrigger {
    pub amount: u32,
}

/// calls f(body,body,pair) for each collision
pub(crate) fn for_each_collision(
    mut reader: MessageReader<CollisionStart>, //read when a collision message is sent
    collisions: Collisions,
    mut f: impl FnMut(Entity, Entity, &ContactPair),
) {
    for collision_event in reader.read() {
        let Some(pair) = collisions.get(collision_event.collider1, collision_event.collider2)
        else {
            continue;
        };

        let (Some(body_a), Some(body_b)) = (collision_event.body1, collision_event.body2) else {
            continue;
        };
        f(body_a, body_b, pair);
    }
}

pub fn match_pair(
    a: Entity,
    b: Entity,
    is_first: impl Fn(Entity) -> bool,
    is_second: impl Fn(Entity) -> bool,
) -> Option<(Entity, Entity)> {
    if is_first(a) && is_second(b) {
        Some((a, b))
    } else if is_first(b) && is_second(a) {
        Some((b, a))
    } else {
        None
    }
}

/// observe any collisions on bodies with health
pub fn observe_collisions(
    mut reader: MessageReader<CollisionStart>, //read when a collision message is sent
    mut writer: MessageWriter<DamageEvent>,    //write out a damage event
    collisions: Collisions,
    mut health: Query<(&Mass, &mut Health)>,
) {
    for_each_collision(reader, collisions, |a, b, pair| {
        let impulse = pair.total_normal_impulse_magnitude();

        for body in [a, b] {
            let Ok((mass, mut health)) = health.get_mut(body) else {
                continue;
            };
            if impulse / mass.0 > IMPACT_THRESHOLD {

                let normalized_impulse = impulse / mass.0;
                let damage: u32 = (normalized_impulse / 10.0).round() as u32;
                info!("calculated damage {} from raw impulse {} normalized to {}", damage, impulse, normalized_impulse);
                writer.write(DamageEvent {
                    target: body,
                    amount: damage,
                });

                health.apply_damage(damage);
            }
        }
    });
}
