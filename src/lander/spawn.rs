use crate::lander::lander::Lander;
use crate::world::layers::WorldLayer;
use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::asset::{AssetServer, Handle};
use bevy::log::error;
use bevy::math::Vec3;
use bevy::prelude::{default, Color, Commands, Res, Transform, Vec2, Image, info, Query, Entity, With};
use bevy::sprite::Sprite;

use crate::config::lander;
use crate::lander::physics::ShipStatus;

pub fn spawn_lander(mut commands: Commands, 
                    asset_server: Res<AssetServer>,
query: Query<Entity, With<Lander>>,) {

    let Err(_) = query.single() else {
        error!("there is already a lander!");
        return;
    };

    //TC: no sprite sheets here just a flat image
    let spaceship: Handle<Image> = asset_server.load("noun_spaceship.png");
    
    info!("spawn lander");


    commands.spawn((
        // marker obj
        Lander,
        ShipStatus::new(),
        // position in space
        Transform::from_xyz(lander::LANDER_START_X,  lander::LANDER_START_Y, lander::LANDER_Z_POS),
        // physics
        RigidBody::Dynamic,
        // rendered object
        // Sprite {
        //     color: Color::srgb(0.9, 0.3, 0.3),
        //     custom_size: Some(Vec2::new(lander::LANDER_SIZE, lander::LANDER_SIZE)), //TODO: mark size somehow
        //     ..default()
        // },
        Sprite {
            image: spaceship,
                custom_size: Some(Vec2::new(lander::LANDER_SIZE, lander::LANDER_SIZE)),
            ..default()
        },
        Collider::rectangle(lander::LANDER_SIZE, lander::LANDER_SIZE), //TODO: mark position somehow
        CollisionLayers::new(
            [WorldLayer::Lander], 
            [WorldLayer::Floor, WorldLayer::Package, WorldLayer::Enemies, WorldLayer::Props]),
    ));
}
