use crate::config;
use crate::lander::lander::Lander;
use crate::lander::ship::ShipStatus;
use avian2d::prelude::{Forces, ReadRigidBodyForces, WriteRigidBodyForces};
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Query, Res, With};
use crate::lander::input::InputDirection;

pub fn handle_rotation_thrust(
    mut query: Query<(Forces, &mut ShipStatus), With<Lander>>,
) {
    for (mut forces, mut ship_status) in query.iter_mut() {

        if (!ship_status.use_secondary_thrust()) {
            ship_status.angular_force = 0.0;
            return;
        }

        let rotation_factor = gen_direction_input(ship_status.thrust_direction);

        // angular_velocity.0 += rotation_factor;
        let angular_velocity = forces.angular_velocity();

        ship_status.angular_force = angular_velocity;

        //changing acceleration makes it rotate where impulse does not
        forces.apply_angular_acceleration(rotation_factor)
    }
}
fn gen_direction_input(input: InputDirection) -> f32 {
    let direction: f32 = if (input == InputDirection::Left) {
        1.0
    } else if (input == InputDirection::Right) {
        -1.0
    } else {
        0.0
    };
    direction * config::lander::ROTATION_FORCE
}
