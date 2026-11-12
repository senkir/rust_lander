use crate::camera::trauma::CameraShake;
use crate::damage::death::DeathKind;
use crate::particles;
use crate::particles::materials::{FireParticleMaterial, SmokeParticleMaterial};
use bevy::asset::AssetServer;
use bevy::log::info;
use bevy::prelude::{
    Assets, Commands, Entity, GlobalTransform, Message, MessageReader, Query, Res, ResMut,
    Transform,
};
use bevy_enoki::prelude::OneShot;
use bevy_enoki::{ParticleEffectHandle, ParticleSpawner};

#[derive(Message)]
pub struct DeathTrigger {
    pub entity: Entity,
    pub kind: DeathKind,
}
pub fn spawn_explosion(
    mut reader: MessageReader<DeathTrigger>,
    transforms: Query<&GlobalTransform>,
    mut commands: Commands,
    mut fire_mat: ResMut<Assets<FireParticleMaterial>>,
    mut shake: ResMut<CameraShake>,
    server: Res<AssetServer>,
) {
    for (death_trigger) in reader.read() {
        info!("death trigger: {:?}", death_trigger.entity);
        let Ok(pos) = transforms.get(death_trigger.entity) else {
            continue;
        };

        //FIXME: camera shake is a problem
        if (death_trigger.kind == DeathKind::Lander) {
            // shake.trauma = 0.3;
        } else {
            // shake.trauma = 0.1;
        }

        let at = pos
            .translation()
            .with_z(crate::config::lander::LANDER_Z_POS + 0.5); //todo; is this right z

        commands.spawn((
            ParticleSpawner(fire_mat.add(FireParticleMaterial::default())),
            // ParticleSpawner::default(),
            ParticleEffectHandle(server.load(particles::particles::PARTICLE_EXPLOSION_SPARKS)),
            OneShot::Despawn,
            Transform::from_translation(at),
        ));

        commands.entity(death_trigger.entity).with_child(
            // commands.spawn((
            (
                ParticleSpawner(fire_mat.add(FireParticleMaterial::default())),
                ParticleEffectHandle(server.load(particles::particles::PARTICLE_EXPLOSION_FIRE)),
                OneShot::Despawn,
                // Transform::from_translation(at)
                Transform::from_xyz(0.0, 0.0, 1.0),
            ),
        );

    }
}
