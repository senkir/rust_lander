use crate::anim;
use crate::anim::{AnimationIndices, AnimationMode, AnimationTimer};
use crate::config::{lander, sprites};
use crate::damage::death::Dying;
use crate::damage::health::Health;
use crate::deliveries::delivery::Delivery;
use crate::deliveries::landing_pad::collision::DeliveryEventTrigger;
use crate::deliveries::{config, landing_pad};
use crate::hud::score::display::ScoreUpdate;
use crate::lander::assets::SpriteAtlasHandles;
use crate::world::layers::WorldLayer;
use crate::world::spawn::World;
use avian2d::prelude::{Collider, CollisionEventsEnabled, CollisionLayers, Mass, RigidBody};
use bevy::log::info;
use bevy::prelude::{
    AssetServer, Commands, Entity, Message, MessageReader, MessageWriter, Query, Res, Sprite,
    Transform, With, Without,
};
use rand::RngExt;

#[derive(Message, Default)]
pub struct SpawnPackageRequest(u32);

impl SpawnPackageRequest {
    pub fn new(count: u32) -> Self {
        SpawnPackageRequest(count)
    }
}

pub fn spawn_package(
    mut commands: Commands,
    world: Res<World>,
    sprite_atlas_handles: Res<SpriteAtlasHandles>,
    asset_server: Res<AssetServer>,
) {
    spawn_one_package(&mut commands, &world, &sprite_atlas_handles, &asset_server);
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

fn spawn_one_package(
    commands: &mut Commands,
    world: &World,
    sprite_atlas_handles: &SpriteAtlasHandles,
    asset_server: &AssetServer,
) {
    let package_img = sprite_atlas_handles.get(sprites::SPRITE_PACKAGE);

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

pub fn check_dying(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Sprite),
        (With<Dying>, With<Delivery>, Without<anim::AnimationIndices>),
    >,
) {
    for (entity, mut sprite) in query.iter_mut() {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            return;
        };
        //cow is dying but has no animation running
        info!("starting death animation");

        let anim = AnimationIndices::new(8, 14, AnimationMode::Once);
        commands
            .entity(entity).insert(anim);
        commands
            .entity(entity)
            .insert(AnimationTimer::from_seconds(0.1));
    }
}

pub fn despawn_package(
    mut commands: Commands,
    mut writer: MessageWriter<SpawnPackageRequest>,
    mut score_writer: MessageWriter<ScoreUpdate>,
    mut reader: MessageReader<DeliveryEventTrigger>,
    deliveries_query: Query<Entity, With<Delivery>>,
) {
    if reader.len() == 0 {
        return;
    }

    let mut delivered: usize = 0;
    for (delivery_event) in reader.read() {
        //some events occurred

        commands.entity(delivery_event.entity).try_despawn();
        delivered += 1;

        if (delivery_event.success) {
            score_writer.write(ScoreUpdate::new(landing_pad::collision::PACKAGE_POINTS));
        } else {
            score_writer.write(ScoreUpdate::new(
                -landing_pad::collision::PACKAGE_DESTROYED_POINTS,
            ));
        }
    }
    let remaining = deliveries_query.count().saturating_sub(delivered);

    if remaining == 0 {
        info!("Spawning more packages!!");
        writer.write(SpawnPackageRequest::new(5));
    }
}
