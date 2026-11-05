use avian2d::prelude::{AngularVelocity, Collider, Forces, ReadRigidBodyForces, RigidBody, WriteRigidBodyForces};
use avian2d::prelude::forces::ForcesItem;
use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::prelude::{Component, KeyCode, Query, Res, Single, Transform, Vec3, With};
use bevy::time::Time;
use crate::config;
use crate::lander::lander::Lander;
use crate::lander::physics::ShipStatus;

#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputDirection {
    Left,
    Right,
    #[default]
    None
}
pub fn handle_rotation_thrust(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Forces, &mut ShipStatus), With<Lander>>,
    // query: Single<(&mut AngularVelocity), With<Lander>>,
) {

    // let (mut angular_velocity) = query.into_inner();


    for (mut forces, mut ship_status) in query.iter_mut() {
        //TODO: TC: trying to apply an angular impulse instead of setting a rotation position

        ship_status.is_rotating = is_rotation_requested(&input);

        if (!ship_status.is_rotating) {
            // info!("not rotating");
            return;
        }

        let rotation_factor = gen_direction_input(&input);

        // angular_velocity.0 += rotation_factor;
        let angular_velocity = forces.angular_velocity();

        ship_status.angular_force = angular_velocity;

        info!("applying rotational thrust! angular_velocity(existing): {}, rotation_factor: {}", angular_velocity, rotation_factor);
        //changing accelleration makes it rotate where implulse does not
        forces.apply_angular_acceleration(rotation_factor)
        // forces.apply_angular_impulse(rotation_factor)
    }


}

fn is_rotation_requested(input: &ButtonInput<KeyCode>) -> bool {
    input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::ArrowRight)
}
fn gen_direction_input(input: &ButtonInput<KeyCode>) -> f32 {
    let direction: f32 = if (input.pressed(KeyCode::ArrowLeft)) {
         1.0
    } else if (input.pressed(KeyCode::ArrowRight)) {
         -1.0
    } else {
        0.0
    };
    direction * config::lander::ROTATION_SPEED
}
