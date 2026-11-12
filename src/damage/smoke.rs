use avian2d::prelude::{CollisionStart, Collisions, ContactPair};
use bevy::asset::{AssetServer, Assets};
use bevy::log::info;
use bevy::prelude::{Commands, Entity, GlobalTransform, MessageReader, Query, Res, ResMut, Transform, With};
use bevy_enoki::{ParticleEffectHandle, ParticleSpawner};
use bevy_enoki::prelude::OneShot;
use crate::damage::collision;
use crate::damage::collision::match_pair;
use crate::deliveries::delivery::Delivery;
use crate::particles;
use crate::particles::materials::SmokeParticleMaterial;
use crate::world::terrain::TerrainMarker;

pub fn observe_terrain_collisions(
    mut commands: Commands,
    reader: MessageReader<CollisionStart>, //read when a collision message is sent
    collisions: Collisions,
    transforms: Query<&GlobalTransform>,
    terrain: Query<(), With<TerrainMarker>>,
    mut smoke_mat: ResMut<Assets<SmokeParticleMaterial>>,
    server: Res<AssetServer>,

) {
    collision::for_each_collision(
        reader,
        collisions,
        |a, b, collision| {

            let moving_entity: Entity = if terrain.contains(a) {
                b
            } else if terrain.contains(b) {
                a
            } else {
                //no matching collision
                return;
            };
            info!("terrain collision");
            let impulse = collision.total_normal_impulse_magnitude();

            let is_small = impulse < 50_000.0;
            info!("impulse: {:?} small? {}", impulse, is_small);

            let Ok(pos) = transforms.get(moving_entity) else {
                return;
            };

            //spawn smoke
            //match happened so lets spawn
            spawn_smoke(
                &mut commands,
                &pos,
                &mut smoke_mat,
                &server,
                is_small
            );
        }
    );

}

fn spawn_smoke(
    commands: &mut Commands,
    pos: &GlobalTransform,
    smoke_mat: &mut Assets<SmokeParticleMaterial>,
    server: &AssetServer,
    is_small: bool,
) {

    let at = pos
        .translation()
        .with_z(crate::config::lander::LANDER_Z_POS + 0.5); //todo; is this right z

    info!("spawning smoke");
    commands.spawn(
        // commands.spawn((
        (
            ParticleSpawner(smoke_mat.add(SmokeParticleMaterial::default())),
            ParticleEffectHandle(server.load(
                if is_small {
                    particles::particles::PARTICLE_EXPLOSION_SMOKE_SMALL
                } else {
                    particles::particles::PARTICLE_EXPLOSION_SMOKE
                }
            )),
            OneShot::Despawn,
            Transform::from_translation(at),
        ),
    );
}