use crate::lander::lander::Lander;
use crate::lander::ship::ShipStatus;
use bevy::input::ButtonInput;
use bevy::prelude::{Component, KeyCode, Query, Res, With};
use std::fmt::Display;

fn button_to_input_direction(input: &ButtonInput<KeyCode>) -> InputDirection {
    if input.pressed(KeyCode::ArrowUp) {
        return InputDirection::Forward;
    } else if input.pressed(KeyCode::ArrowDown) {
        return InputDirection::Backward;
    } else if input.pressed(KeyCode::ArrowLeft) {
        return InputDirection::Left;
    } else if input.pressed(KeyCode::ArrowRight) {
        return InputDirection::Right;
    } else {
        return InputDirection::None;
    }
}

pub fn observe_thruster_pressed(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut ShipStatus), With<Lander>>,
) {
    for (mut ship_status) in query.iter_mut() {
        let direction = button_to_input_direction(&input);

        match direction {
            InputDirection::Forward => {
                ship_status.thrust_direction = InputDirection::Forward;
            }
            InputDirection::Left => {
                ship_status.thrust_direction = InputDirection::Left;
            }
            InputDirection::Right => {
                ship_status.thrust_direction = InputDirection::Right;
            }
            InputDirection::Backward => {
                ship_status.thrust_direction = InputDirection::Backward;
            }
            InputDirection::None => {
                ship_status.thrust_direction = InputDirection::None;
                //nothing
            }
        }
    }
}

#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum InputDirection {
    Left,
    Right,
    Backward,
    Forward,
    #[default]
    None,
}

impl Display for InputDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
