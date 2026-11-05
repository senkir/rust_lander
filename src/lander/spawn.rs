use crate::lander::lander::Lander;
use crate::lander::physics::Velocity;
use crate::world::layers::WorldLayer;
use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::math::Vec3;
use bevy::prelude::{Color, Commands, Transform, Vec2, default};
use bevy::sprite::Sprite;

const LANDER_SIZE: f32 = 50.0;
pub fn spawn_lander(mut commands: Commands) {
    commands.spawn((
        // marker obj
        Lander,
        // position in space
        Transform::from_xyz(0.0, 0.0, 0.0),
        // physics
        RigidBody::Dynamic,
        // rendered object
        Sprite {
            color: Color::srgb(0.9, 0.3, 0.3),
            custom_size: Some(Vec2::new(LANDER_SIZE, LANDER_SIZE)), //TODO: mark size somehow
            ..default()
        },
        Collider::rectangle(LANDER_SIZE, LANDER_SIZE), //TODO: mark position somehow
        CollisionLayers::new([WorldLayer::Enemies], [WorldLayer::Floor]),
    ));
}
