use std::collections::HashSet;
use crate::config::{lander, sprites};
use crate::world::layers::WorldLayer;
use avian2d::parry::glamx::Vec2;
use avian2d::prelude::{Collider, CollisionEventsEnabled, CollisionLayers, Mass, RigidBody};
use bevy::asset::Handle;
use bevy::image::Image;
use bevy::log::info;
use bevy::prelude::{default, AssetServer, Commands, Query, Res, Sprite, Transform, Entity, With, MessageReader, Message, MessageWriter};
use rand::{RngExt, SeedableRng};
use crate::damage::health::Health;
use crate::deliveries::config;
use crate::deliveries::delivery::Delivery;
use crate::delivery_target::collision::PackageDeliveredTrigger;
use crate::lander::assets::SpriteAtlasHandles;
use crate::score::display::ScoreUpdate;
use crate::world::spawn::World;

#[derive(Message, Default)]
pub struct SpawnPackageRequest(u32);

impl SpawnPackageRequest {
    pub fn new(count: u32) -> Self {
        SpawnPackageRequest(count)
    }
}

pub fn spawn_package(mut commands: Commands,
                     world: Res<World>,    
                     sprite_atlas_handles: Res<SpriteAtlasHandles>,
                     asset_server: Res<AssetServer>) {
    spawn_one_package(
        &mut commands,
        &world,
        &sprite_atlas_handles,
        &asset_server
    );
}



pub fn handle_spawn_requests(
    mut requests: MessageReader<SpawnPackageRequest>,
    mut commands: Commands,
    world: Res<World>,
    sprite_atlas_handles: Res<SpriteAtlasHandles>,
    asset_server: Res<AssetServer>,
) {

    for request in requests.read() {
        for _ in 0..request.0 {
            spawn_one_package(&mut commands, &world, &sprite_atlas_handles, &asset_server);
        }
    }
}

fn spawn_one_package(commands: &mut Commands,
                     world: &World,
                     sprite_atlas_handles: &SpriteAtlasHandles,
                     asset_server: &AssetServer) {
    
    let package_img = sprite_atlas_handles.get(
        sprites::SPRITE_PACKAGE
    );

    let mut rng = rand::rng();
    let pos_x = rng.random_range(-500.0..world.right());
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
        package_img.unwrap().sprite(0),
        // Sprite {
        //     image: package_img,
        //     custom_size: Some(Vec2::new(lander::LANDER_SIZE, lander::LANDER_SIZE)),
        //     ..default()
        // },
        CollisionEventsEnabled,
        Mass(config::MASS),
        Collider::rectangle(lander::LANDER_SIZE, lander::LANDER_SIZE), //TODO: mark position somehow
        CollisionLayers::new(
            [WorldLayer::Package],
            [
                WorldLayer::Floor,
                WorldLayer::Lander,
                WorldLayer::Target,
                WorldLayer::Enemies,
                WorldLayer::Props,
            ],
        ),
    ));
}

pub fn despawn_package(
    mut commands: Commands,
    mut writer: MessageWriter<SpawnPackageRequest>,
    mut score_writer: MessageWriter<ScoreUpdate>,
    mut reader: MessageReader<PackageDeliveredTrigger>,
    deliveries_query: Query<Entity, With<Delivery>>,
) {
    if reader.len() == 0 {
        return;
    }
    let delivered: HashSet<Entity> = reader.read().map(|e| e.0).collect();

    for entity in &delivered {
        commands.entity(*entity).try_despawn();
        score_writer.write(ScoreUpdate::new(crate::delivery_target::collision::PACKAGE_POINTS));

    }
    let remaining = deliveries_query.count().saturating_sub(delivered.len());

    if remaining == 0 {
        info!("Spawning more packages!!");
        writer.write(SpawnPackageRequest::new(5));
    }

}