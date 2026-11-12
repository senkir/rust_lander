use crate::lander::respawn::RespawnBeginMessage;
use crate::state::GameState;
use crate::state::button::button;
use bevy::ecs::children;
use bevy::log::info;
use bevy::prelude::{
    AlignItems, BackgroundColor, Color, Commands, Component, Entity, FontSize, Justify,
    JustifyContent, MessageWriter, NextState, Node, On, Query, ResMut, Text, TextColor, TextFont,
    TextLayout, Val, With, default,
};
use bevy::ui::FlexDirection;
use bevy::ui_widgets::{Activate, observe};

#[derive(Component)]
pub struct DeathScreen;

fn trigger_respawn(
    mut writer: MessageWriter<RespawnBeginMessage>,
    next_state: &mut NextState<GameState>,
) {
    writer.write(RespawnBeginMessage);
    next_state.set(GameState::Playing);
}

pub fn spawn_death_screen(mut commands: Commands) {
    info!("spawning death screen");
    commands.spawn((
        DeathScreen,
        //main container
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![((
            //menu container
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            children![
                (
                    Text::new("YOU DIED"),
                    TextFont {
                        font_size: FontSize::Px(36.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextLayout::justify(Justify::Center),
                ),
                (button("Respawn"),
                observe(
                    |_activate: On<Activate>,
                     writer: MessageWriter<RespawnBeginMessage>,
                     mut next_state: ResMut<NextState<GameState>>| {
                        info!("respawn requested");
                        trigger_respawn(writer, &mut next_state);
                    }
                ))
            ],
        ),)],
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
    ));
}

pub fn despawn_death_screen(mut commands: Commands, query: Query<Entity, With<DeathScreen>>) {
    info!("despawned death screen");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
