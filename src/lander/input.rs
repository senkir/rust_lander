use avian2d::prelude::Forces;
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Query, Res, With};
use crate::lander::lander::Lander;
use crate::lander::ship::ShipStatus;

fn read_thrust_input(input: &ButtonInput<KeyCode>) -> bool {
    input.pressed(KeyCode::ArrowUp)
}


pub fn observe_thruster_pressed(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut ShipStatus), With<Lander>>,
) {
    let wants_boost = read_thrust_input(&input);

    for (mut ship_status) in query.iter_mut() {
        //update boosting
        ship_status.is_thrusting = wants_boost;
    }
}