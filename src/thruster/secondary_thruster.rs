use crate::config;
use crate::lander::lander::Lander;
use crate::lander::ship::ShipStatus;
use avian2d::prelude::{Forces, ReadRigidBodyForces, WriteRigidBodyForces};
use avian2d::prelude::forces::ForcesItem;
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Query, Res, With};
use crate::config::lander;
use crate::lander::input::InputDirection;
use crate::thruster::math;

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
        
        let offset_angle: f32 = if (ship_status.thrust_direction == InputDirection::Right) {
            -90.0
        } else if (ship_status.thrust_direction == InputDirection::Left) {
            90.0
        } else {
            return;
        };

        //changing acceleration makes it rotate where impulse does not
        forces.apply_angular_acceleration(rotation_factor);
        
        apply_side_thrust(&mut forces, offset_angle);
    }
}


fn apply_side_thrust(forces: &mut ForcesItem, angle_offset_degrees: f32) {
    //TODO: calculate force from orientation
    //for now just always apply a vertical thrust
    let thruster_force = lander::THRUSTER_FORCE; //TODO: is this too much?

    //shift 90 degrees
    let rot = (forces.rotation().as_degrees() + angle_offset_degrees).to_radians();

    //TODO: run this calculation
    let vector = math::calculate_thrust_vector(thruster_force, rot);

    forces.apply_linear_acceleration(vector);
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
