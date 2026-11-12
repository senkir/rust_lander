use avian2d::prelude::{CollisionStart, Collisions, Mass};
use bevy::log::info;
use bevy::prelude::{Entity, Message, MessageReader, MessageWriter, Query, With};
use crate::damage::collision;
use crate::damage::collision::{match_pair, DamageEvent};
use crate::damage::health::Health;
use crate::deliveries::delivery::Delivery;
use crate::delivery_target::spawn::LandingPad;
use crate::lander::lander::Lander;
use crate::score::display::ScoreUpdate;

#[derive(Message, Debug, Clone)]
pub struct PackageDeliveredTrigger(pub Entity);

pub(crate) const PACKAGE_POINTS: u32 = 100;

pub fn observe_package_collisions(
    reader: MessageReader<CollisionStart>, //read when a collision message is sent
    mut writer: MessageWriter<PackageDeliveredTrigger>,    //write out a damage event
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
            writer.write(PackageDeliveredTrigger(package));
        }
    );

}