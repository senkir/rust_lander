use crate::config::lander;
use crate::damage::health::Health;
use crate::lander::ship::ShipStatus;
use crate::lander::lander::Lander;
use crate::world::layers::WorldLayer;
use avian2d::prelude::{AngularDamping, Collider, CollisionEventsEnabled, CollisionLayers, LinearDamping, Mass, RigidBody};
use bevy::asset::{AssetServer, Handle};
use bevy::log::error;
use bevy::math::Vec3;
use bevy::prelude::{
    Color, Commands, Entity, Image, Query, Res, Transform, Vec2, With, default, info,
};
use bevy::sprite::Sprite;
use crate::beam::ship::{ShipTractor, TractorState};

pub fn spawn_lander(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<Lander>>,
) {
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
        Health::new(100),
        // position in space
        Transform::from_xyz(
            lander::LANDER_START_X,
            lander::LANDER_START_Y,
            lander::LANDER_Z_POS,
        ),
        // physics
        RigidBody::Dynamic,

        //TODO: make this dynamic per environment
        LinearDamping(crate::config::world::LINEAR_DRAG),   // slows translation, like drag
        AngularDamping(crate::config::world::ANGULAR_DRAG),  // slows spin
        // rendered object
        CollisionEventsEnabled,
        ShipTractor::default(),
        Mass(lander::MASS),
        Sprite {
            image: spaceship,
            custom_size: Some(Vec2::new(lander::LANDER_SIZE, lander::LANDER_SIZE)),
            ..default()
        },
        Collider::rectangle(lander::LANDER_SIZE, lander::LANDER_SIZE), //TODO: mark position somehow
        CollisionLayers::new(
            [WorldLayer::Lander],
            [
                WorldLayer::Floor,
                WorldLayer::Package,
                WorldLayer::Target,
                WorldLayer::Enemies,
                WorldLayer::Props,
            ],
        ),
    ));
}
