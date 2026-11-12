use crate::config::hud;
use crate::score::score::Score;
use bevy::color::Color;
use bevy::prelude::{
    Commands, Component, FontSize, Message, MessageReader, Node, PositionType, Query, ResMut, Text,
    TextColor, TextFont, Val, With, default,
};

const SCORE_COLOR: Color = Color::WHITE;

#[derive(Component)]
pub struct ScoreText;

/// message to update score by a certain amount
#[derive(Component, Message)]
pub struct ScoreUpdate(u32);

impl ScoreUpdate {
    pub fn new(score: u32) -> Self {
        Self(score)
    }
}

fn build_score(score: u32) -> String {
    //TODO: localize?
    format!("Score: {:?}", score)
}

pub fn spawn_score(mut commands: Commands, score: ResMut<Score>) {
    commands.spawn((
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
