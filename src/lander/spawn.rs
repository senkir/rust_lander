use crate::lander::lander::Lander;
use crate::world::layers::WorldLayer;
use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::math::Vec3;
use bevy::prelude::{Color, Commands, Transform, Vec2, default};
use bevy::sprite::Sprite;

use crate::config::lander;
use crate::lander::physics::ShipStatus;

pub fn spawn_lander(mut commands: Commands) {
    commands.spawn((
        // marker obj
        Lander,
        ShipStatus::new(),
        // position in space
        Transform::from_xyz(lander::LANDER_START_X,  lander::LANDER_START_Y, lander::LANDER_Z_POS),
        // physics
        RigidBody::Dynamic,
        // rendered object
        Sprite {
            color: Color::srgb(0.9, 0.3, 0.3),
            custom_size: Some(Vec2::new(lander::LANDER_SIZE, lander::LANDER_SIZE)), //TODO: mark size somehow
            ..default()
        },
        Collider::rectangle(lander::LANDER_SIZE, lander::LANDER_SIZE), //TODO: mark position somehow
        CollisionLayers::new(
            [WorldLayer::Lander], 
            [WorldLayer::Floor, WorldLayer::Enemies, WorldLayer::Props]),
    ));
}
