use crate::damage::collision;
use crate::damage::collision::{DamageEvent, match_pair};
use crate::damage::health::Health;
use crate::deliveries::delivery::Delivery;
use crate::deliveries::landing_pad::spawn::LandingPad;
use crate::hud::score::display::ScoreUpdate;
use crate::lander::lander::Lander;
use avian2d::prelude::{CollisionStart, Collisions, Mass};
use bevy::log::info;
use bevy::prelude::{Entity, Message, MessageReader, MessageWriter, Query, With};

#[derive(Message, Debug, Clone)]
pub struct DeliveryEventTrigger {
    pub entity: Entity,
    pub success: bool
}

impl DeliveryEventTrigger {
    pub fn new(entity: Entity) -> DeliveryEventTrigger {
        Self {
            entity,
            success: true
        }
    }
}

pub(crate) const PACKAGE_POINTS: i32 = 100;
pub(crate) const PACKAGE_DESTROYED_POINTS: i32= 50;

pub fn observe_package_collisions(
    reader: MessageReader<CollisionStart>, //read when a collision message is sent
    mut writer: MessageWriter<DeliveryEventTrigger>,    //write out a damage event
    collisions: Collisions,
    landing_pad_entity: Query<(), With<LandingPad>>,
    packages_entity: Query<(), With<Delivery>>,
) {

    collision::for_each_collision(
        reader,
        collisions,
        |a,b, _pair| {
            let Some((package, _pad)) = match_pair(
                a, b,
                |e| packages_entity.contains(e),
                |e| landing_pad_entity.contains(e),
            ) else { return; };

            info!("package delivered!!");
            writer.write(DeliveryEventTrigger::new(package));
        }
    );

}