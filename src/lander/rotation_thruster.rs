use crate::config;
use crate::lander::ship::ShipStatus;
use crate::lander::lander::Lander;
use avian2d::prelude::{
    AngularVelocity, Collider, Forces, ReadRigidBodyForces, RigidBody, WriteRigidBodyForces,
};
use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::prelude::{Component, KeyCode, Query, Res, Single, Transform, Vec3, With};
use bevy::time::Time;

#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputDirection {
    Left,
    Right,
    #[default]
    None,
}
pub fn handle_rotation_thrust(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Forces, &mut ShipStatus), With<Lander>>,
) {
    for (mut forces, mut ship_status) in query.iter_mut() {
        //TODO: TC: trying to apply an angular impulse instead of setting a rotation position

        ship_status.is_rotating = is_rotation_requested(&input);

        let rotation_factor = gen_direction_input(&input);

        // angular_velocity.0 += rotation_factor;
        let angular_velocity = forces.angular_velocity();

        ship_status.angular_force = angular_velocity;

        //changing acceleration makes it rotate where impulse does not
        forces.apply_angular_acceleration(rotation_factor)
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
