use crate::beam::ship::{ShipTractor, TractorState};
use crate::config;
use crate::config::{lander, sprites};
use crate::damage::health;
use crate::damage::health::Health;
use crate::lander::assets::{SpriteAtlasHandles, TileMapHandles};
use crate::lander::lander::Lander;
use crate::lander::respawn::{RespawnReadyMessage, RespawnBeginMessage};
use crate::lander::ship::ShipStatus;
use crate::world::layers::WorldLayer;
use avian2d::prelude::{AngularDamping, Collider, CollisionEventsEnabled, CollisionLayers, LinearDamping, Mass, RigidBody};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::log::error;
use bevy::math::Vec3;
use bevy::prelude::{Color, Commands, Entity, Image, MessageReader, MessageWriter, Query, Res, ResMut, TextureAtlas, TextureAtlasLayout, Transform, UVec2, Vec2, With, default, info};
use bevy::sprite::Sprite;
use crate::damage::damage_stage::DamageStage;

pub fn spawn_lander(
    mut commands: Commands,
    sprite_atlas_handles: Res<SpriteAtlasHandles>,
    query: Query<Entity, With<Lander>>) {

    do_spawn_lander(
        &mut commands, &sprite_atlas_handles, query
    );
}

fn do_spawn_lander(
    commands: &mut Commands,
    sprite_atlas_handles: &SpriteAtlasHandles,
    query: Query<Entity, With<Lander>>,
) {
    let Err(_) = query.single() else {
        error!("there is already a lander!");
        return;
    };

    //TC: no sprite sheets here just a flat image

    //TODO: load this as a sprite map
    //TODO: is this clone efficient? who knows
    let Some(spaceship) = sprite_atlas_handles.get(sprites::SPRITE_LANDER) else {
        error!("failed to get sprite spaceship!");
        return;
    };

    commands.spawn((
        // marker obj
        Lander,
        ShipStatus::new(),
        Health::new(lander::LANDER_HEALTH),
        DamageStage::default(),
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
        spaceship.sprite(
            0
        ),
        Collider::rectangle(lander::LANDER_SIZE, lander::LANDER_SIZE),
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

pub fn update_lander_frame(
    mut query: Query<(&Health, &DamageStage, &ShipStatus, &mut Sprite), With<Lander>>,
) {
    let Ok((health, damage_stage, status, mut sprite)) = query.single_mut() else {
        return;
    };

    let Some(atlas) = sprite.texture_atlas.as_mut() else {
        return;
    };

    let index = damage_stage.damage_stage(health);
    if atlas.index != index {
        atlas.index = index;
    }
}

pub fn observe_respawn_start(
    mut commands: Commands,
    query: Query<Entity, With<Lander>>,
    mut reader: MessageReader<RespawnBeginMessage>,
    mut writer: MessageWriter<RespawnReadyMessage>
) {
    //FIXME: there's probably a cleaner way to do this but oh well

    for event in reader.read() {
        //only get here when there's an event to trigger

        let Ok(entity) = query.single() else {
            error!("there is no lander!");
            return;
        };

        commands.entity(entity).despawn();

        writer.write(RespawnReadyMessage);
    }
}

pub fn observe_respawn_ready(
    mut commands: Commands,
    mut reader: MessageReader<RespawnReadyMessage>,
    sprite_atlas_handles: Res<SpriteAtlasHandles>,
    query: Query<Entity, With<Lander>>,
) {

    for event in reader.read() {
    do_spawn_lander(
        &mut commands,
        &sprite_atlas_handles,
        query
    );
    }
}


#[cfg(test)]
mod Test {
    use super::*;

    #[test]
    fn test_fraction() {
        let mut health = Health::default();
        assert_eq!(health.fraction(), 1.0);
    }
    #[test]
    fn test_mark_for_death() {
        let mut health = Health::default();
        health.mark_for_death();
        assert!(health.is_dying());
    }

    #[test]
    fn test_dead_not_dying() {
        let mut health = Health::default();
        let result = health.is_dead_not_dying();
        assert_eq!(result, false);
    }
}