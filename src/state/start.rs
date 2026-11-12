use crate::lander::respawn::RespawnBeginMessage;
use crate::state::GameState;
use crate::state::button::button;
use bevy::ecs::children;
use bevy::log::info;
use bevy::picking::hover::Hovered;
use bevy::prelude::{
    AlignItems, AssetServer, BackgroundColor, BorderRadius, Bundle, Color, Commands, Component,
    DespawnOnExit, Entity, FontSize, Justify, JustifyContent, MessageWriter, NextState, Node, On,
    Query, Res, ResMut, Text, TextColor, TextFont, TextLayout, TextShadow, Val, With, default,
};
use bevy::ui::{BorderColor, FlexDirection, UiRect, px};
use bevy::ui_widgets::{Activate, Button, observe};

pub fn spawn_ready_screen(mut commands: Commands) {
    info!("spawning ready screen");
    commands.spawn((
        DespawnOnExit(GameState::Ready),
        //main container
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            //inner menu
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            children![
                (
                    Text::new("Ready"),
                    TextFont {
                        font_size: FontSize::Px(36.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextLayout::justify(Justify::Center),
                ),
                (button("Play"),
                observe(
                    |_activate: On<Activate>, mut next_state: ResMut<NextState<GameState>>| {
                        info!("play pressed");
                        next_state.set(GameState::Playing);
                    }
                ))
            ],
        )],
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
    ));
}
