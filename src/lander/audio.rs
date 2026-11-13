use bevy::asset::AssetServer;
use bevy::audio::{AudioPlayer, PlaybackSettings};
use bevy::prelude::{Commands, Component, Entity, Message, MessageReader, Query, Res, With};

#[derive(Component)]
pub struct EnginePlayback;

#[derive(Message)]
pub struct EngineMessage(bool);

pub fn add_engine_sound(
    mut commands: Commands,
    mut reader: MessageReader<EngineMessage>,
    query: Query<Entity, With<EnginePlayback>>,
    asset_server: Res<AssetServer>,
) {
    for event in reader.read() {
        if event.0 {
            commands.spawn((
                EnginePlayback,
                AudioPlayer::new(asset_server.load("audio/tiny_motor.wav")),
                PlaybackSettings::LOOP,
            ));
        } else {
            for entity in query.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}
