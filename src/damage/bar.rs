use bevy::color::Color;
use bevy::prelude::{default, BackgroundColor, BorderColor, BorderRadius, Commands, Component, Node, PositionType, UiRect, Val, Query, With, MessageReader, ResMut};
use bevy::ui::Overflow;
use crate::damage::health::Health;
use crate::lander::lander::Lander;
use crate::lander::respawn::RespawnReadyMessage;

#[derive(Component)]
pub struct HealthBarFill;


pub fn spawn_lander_health_bar(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(crate::config::hud::HEALTH_POSITION_Y),
            left: Val::Px(12.0),
            width: Val::Px(180.0),
            height: Val::Px(20.0),
            border: UiRect::all(Val::Px(2.0)), // optional outline
            border_radius: BorderRadius::all(Val::Px(12.0)), // curved corners
            overflow: Overflow::clip(),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        BorderColor::all(Color::WHITE), // only if you set `border`
    )).with_child((
        HealthBarFill,
        Node {
            width: Val::Percent(100.0),                // changed every frame
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.9, 0.3)),
    ));
}

pub fn update_lander_health(
    health: Query<&Health, With<Lander>>,
    mut fill: Query<(&mut Node, &mut BackgroundColor), With<HealthBarFill>>) {
    let Ok(health) = health.single() else {
        return;
    };
    let f = health.fraction();
    
    for (mut node, mut color) in fill.iter_mut() {
        node.width = Val::Percent(f * 100.0);
        
        color.0 = Color::srgb(1.0 - f, 0.2 + 0.7 * f, 0.2);
    }
}