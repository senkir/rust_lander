use crate::config::lander;
use crate::lander::lander::Lander;
use crate::lander::ship::ShipStatus;
use crate::world::layers::WorldLayer;
use avian2d::parry::glamx::Vec2;
use avian2d::prelude::{Collider, CollisionEventsEnabled, CollisionLayers, Mass, RigidBody};
use bevy::asset::Handle;
use bevy::asset::uuid::Version::Random;
use bevy::image::Image;
use bevy::prelude::{AssetServer, Commands, Res, Sprite, Transform, default};
use rand::rngs::SmallRng;
use rand::{RngExt, SeedableRng};
use crate::damage::health::Health;
use crate::deliveries::config;
use crate::deliveries::delivery::Delivery;
use crate::world::layers::WorldLayer::Package;

pub fn spawn_package(mut commands: Commands, asset_server: Res<AssetServer>) {
    //TC: no sprite sheets here just a flat image
    let package_img: Handle<Image> = asset_server.load("noun_package.png");

    //generate random start position

    let mut rng = rand::rng();
    let pos_x = rng.random_range(-500.0..500.0);
    let pos_y = rng.random_range(-200.0..100.0);

    commands.spawn((
        // marker obj
        Delivery,
        // position in space
        Transform::from_xyz(pos_x, pos_y, lander::LANDER_Z_POS),
        // physics
        RigidBody::Dynamic,
        Health::new(100),
        // rendered object
        Sprite {
            image: package_img,
            custom_size: Some(Vec2::new(lander::LANDER_SIZE, lander::LANDER_SIZE)),
            ..default()
        },
        CollisionEventsEnabled,
        Mass(config::MASS),
        Collider::rectangle(lander::LANDER_SIZE, lander::LANDER_SIZE), //TODO: mark position somehow
        CollisionLayers::new(
            [WorldLayer::Package],
            [
                WorldLayer::Floor,
                WorldLayer::Lander,
                WorldLayer::Enemies,
                WorldLayer::Props,
            ],
        ),
    ));
}
