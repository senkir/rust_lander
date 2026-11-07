use crate::config::lander;
use crate::lander::lander::Lander;
use avian2d::parry::glamx::Vec3;
use avian2d::prelude::forces::ForcesItem;
use avian2d::prelude::{Forces, ReadRigidBodyForces, WriteRigidBodyForces};
use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::prelude::{Component, KeyCode, Query, Res, Single, Transform, Vec2, With};

#[derive(Component, Debug)]
pub struct ShipStatus {
    pub is_thrusting: bool,
    pub is_rotating: bool,
    pub angular_force: f32,
    pub velocity: f32,
}

#[derive(Component)]
pub struct ThrustActiveState;

impl ShipStatus {
    pub fn new() -> Self {
        Self {
            is_thrusting: false,
            is_rotating: false,
            angular_force: 0.0,
            velocity: 0.0,
        }
    }
}

fn read_thrust_input(input: &ButtonInput<KeyCode>) -> bool {
    input.pressed(KeyCode::Space)
}

pub fn observe_thruster_pressed(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Forces, &mut ShipStatus), With<Lander>>,
) {
    let wants_boost = read_thrust_input(&input);

    for (mut forces, mut ship_status) in query.iter_mut() {
        //update boosting
        ship_status.is_thrusting = wants_boost;

        if (wants_boost) {
            // info!("wants boost!!!!");

            // Get the ship's forward vector by applying the current rotation to the ships initial facing
            // vector
            // let movement_direction = transform.rotation * Vec3::Y;
            apply_forces(&mut forces);
        }
    }
}

pub fn apply_forces(forces: &mut ForcesItem) {
    //TODO: calculate force from orientation
    //for now just always apply a vertical thrust
    let thruster_force = lander::THRUSTER_FORCE;

    let rot = forces.rotation().as_radians();

    //TODO: run this calculation
    let vector = calculate_thrust_vector(thruster_force, rot);

    // info!("apply_force {}", vector);

    //apply thruster
    // forces.apply_force(vector);
    forces.apply_linear_acceleration(vector);
}

fn calculate_thrust_vector(thruster_force: f32, rotation_radians: f32) -> Vec2 {
    //TODO: run this calculation
    Vec2::new(-rotation_radians.sin(), rotation_radians.cos()) * thruster_force
}

pub fn spawn_thruster() {}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::ops::round;
    use std::f32::consts::PI;

    #[test]
    fn calculate_thrust_90() {
        let test: f32 = 90.0;
        let rad = test.to_radians();
        let result = calculate_thrust_vector(1.0, rad);
        assert!(result.x.abs() == 1.0, "not equal {}", result.x.abs());
        assert!(result.y.abs() < 1e-6, "not equal {}", result.y.abs());
    }

    #[test]
    fn calculate_thrust_0() {
        let test: f32 = 0.0;
        let rad = test.to_radians();
        let result = calculate_thrust_vector(1.0, rad);
        assert!(result.x.abs() == 0.0, "not equal {}", result.x.abs()); //close to 0
        assert!(result.y.abs() == 1.0, "not equal {}", result.y.abs());
    }
}
