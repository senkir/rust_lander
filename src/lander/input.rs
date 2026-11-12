use crate::lander::lander::Lander;
use crate::lander::ship::ShipStatus;
use bevy::input::ButtonInput;
use bevy::prelude::{Component, KeyCode, Query, Res, With};
use std::fmt::Display;
use crate::controls::controls::ControlsResource;

fn button_to_input_direction(input: &ButtonInput<KeyCode>, controls: &ControlsResource) -> InputDirection {
    return if input.pressed(controls.main_engine) {
        InputDirection::Forward
    } else if input.pressed(controls.reverse_engine) {
        InputDirection::Backward
    } else if input.pressed(controls.left_engine) {
        InputDirection::Left
    } else if input.pressed(controls.right_engine) {
        InputDirection::Right
    } else {
        InputDirection::None
    }
}

pub fn observe_thruster_pressed(
    input: Res<ButtonInput<KeyCode>>,
    controls: Res<ControlsResource>,
    mut query: Query<(&mut ShipStatus), With<Lander>>,
) {
    for (mut ship_status) in query.iter_mut() {
        let direction = button_to_input_direction(&input, &controls);

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
