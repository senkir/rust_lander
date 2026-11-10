use bevy::color::{Alpha, Color};
use bevy::ecs::system::entity_command::despawn;
use bevy::math::Vec3;
use bevy::prelude::{default, Commands, Component, GlobalTransform, MessageReader, Query, Text2d, TextColor, TextFont, Timer, Transform, TimerMode, Res, Time, Entity};
use bevy::text::FontSize;
use crate::damage::collision::DamageEvent;

#[derive(Component)]
pub struct FloatingText {
    timer: Timer,
    rise_speed: f32,     // world units per second
}


pub fn spawn_damage_number(
    mut events: MessageReader<DamageEvent>,
    targets: Query<&GlobalTransform>,
    mut commands: Commands,
) {
    for event in events.read() {
        let Ok(target) = targets.get(event.target) else { continue; };

        // above the sprite, in front of it
        //TODO: make sure the Z here is right
        let pos = target.translation() + Vec3::new(0.0, 40.0, 5.0);
        commands.spawn((
            Text2d::new(format!("-{}", event.amount)),
            TextFont { font_size: FontSize::Px(24.0), 
                ..default() 
            },
            TextColor(Color::srgb(1.0, 0.3, 0.3)),
            Transform::from_translation(pos),
            FloatingText {
                timer: Timer::from_seconds(0.8, TimerMode::Once),
                rise_speed: 40.0,
            },
        ));
    }
}

pub fn animate_floating_text(
    time: Res<Time>,
    mut commands: Commands,
    mut texts: Query<(Entity, &mut Transform, &mut TextColor, &mut FloatingText)>,
) {
    for (entity, mut transform, mut color, mut text) in &mut texts {
        text.timer.tick(time.delta());
        transform.translation.y += text.rise_speed * time.delta_secs();
        color.0.set_alpha(1.0 - text.timer.fraction());
        if text.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}