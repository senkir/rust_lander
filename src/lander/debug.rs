use std::fmt::Pointer;
use avian2d::prelude::{AngularVelocity, Forces, LinearVelocity, ReadRigidBodyForces};
use bevy::color::Color;
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Query, Quat, Res, Transform, Vec3, With, Gizmos, Isometry2d, Vec2};
use bevy::text::TextElement::Text;
use crate::lander::lander::Lander;

pub fn debug_reset_sprite(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut LinearVelocity, &mut AngularVelocity), With<Lander>>,
) {
    if !input.just_pressed(KeyCode::Escape) {
        return;
    }
    for (mut transform, mut linear, mut angular) in &mut query {
        transform.translation = Vec3::ZERO;
        transform.rotation = Quat::IDENTITY;
        linear.0 = Default::default();
        angular.0 = 0.0;
    }
}

pub fn debug_show_thrust(
    mut gizmos: Gizmos,
    query: Query<Forces, With<Lander>>,
) {
    let Ok(forces) = query.single() else {
        return;
    };

    let angular_velocity = forces.angular_velocity().round();

    // if angular_velocity < 1e-6
    //     angular_velocity = 0.0

    let velocity = forces.linear_velocity();
    let v_x = velocity.x.round();
    let v_y = velocity.y.round();

    let debug_angular = format!("angular_velocity: {:?}", angular_velocity);
    let debug_linear_x = format!("velocity_x: {}", v_x);
    let debug_linear_y = format!("velocity_y: {}", v_y);

    let text_anchor = Vec2::new(-0.5, 0.5);

    gizmos.text_2d(
        Isometry2d::from_xy(-200.0, 300.0),
        &debug_linear_x,
        12.0,
        text_anchor,
        Color::WHITE,

    );

    gizmos.text_2d(
        Isometry2d::from_xy(-200.0, 300.0 + 30.0),
        &debug_linear_y,
        12.0,
        text_anchor,
        Color::WHITE,

    );

    gizmos.text_2d(
        Isometry2d::from_xy(-200.0, 300.0 + 60.0),
        &debug_angular,
        12.0,
        text_anchor,
        Color::WHITE,

    );
}
