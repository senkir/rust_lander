use crate::config;
use crate::config::hud;
use crate::hud::score::score::Score;
use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::ecs::children;
use bevy::math::Vec2;
use bevy::prelude::{default, Bundle, Commands, Component, FontSize, ImageNode, Message, MessageReader, Node, PositionType, Query, Res, ResMut, Sprite, Text, TextColor, TextFont, Val, With};
use bevy::ui::{AlignItems, FlexDirection};

const SCORE_COLOR: Color = Color::WHITE;

#[derive(Component)]
pub struct ScoreText;

/// message to update score by a certain amount
#[derive(Component, Message)]
pub struct ScoreUpdate(i32);

impl ScoreUpdate {
    pub fn new(score: i32) -> Self {
        Self(score)
    }
}

fn build_score(score: i32) -> String {
    //TODO: localize?
    format!("Score: {:?}", score)
}

pub fn spawn_score(mut commands: Commands,
                   score: ResMut<Score>,
                   asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(hud::SCORE_POSITION_Y),
            left: Val::Px(12.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            player_score(&score),
            control_readout(&asset_server)
        ]
    ));
}

pub fn update_score(
    mut reader: MessageReader<ScoreUpdate>,
    mut score: ResMut<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    for update in reader.read() {
        score.score = score.score + update.0;

        let Ok(mut text) = query.single_mut() else {
            return;
        };

        text.0 = build_score(score.score);
    }
}

fn player_score(score: &Score) -> impl Bundle {
    (
        ScoreText,
        Text::new(build_score(score.score)),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(SCORE_COLOR),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(hud::SCORE_POSITION_Y),
            left: Val::Px(12.0),
            ..default()
        },
    )
}

fn image_node(path: &'static str, asset_server: &AssetServer)-> impl Bundle {
    (
        ImageNode {
        image: asset_server.load(path),
        //TODO: color with usage
        color: Color::srgb(0.8, 0.2, 0.2),
        // custom_size: Some(Vec2::new(32.0, 32.0)),
        ..default()
    },
        Node {
            height: Val::Px(32.0),
            width: Val::Px(32.0),
            ..default()
        }
    )
}
fn control_readout(asset_server: &AssetServer) -> impl Bundle {

    (
        Node {
            position_type: PositionType::Relative,
            align_items: AlignItems::Start,
            ..default()
        },
        // Each sprite goes on its own child entity: a bundle can hold only one
        // `Sprite`, so putting two in the same tuple panics at spawn time.
        children![
            image_node(config::sprites::SPRITE_ARROW, asset_server),
            image_node(config::sprites::SPRITE_LASER, asset_server),
        ],
    )
}
