use crate::deliveries::delivery::Delivery;
use crate::lander::lander::Lander;
use super::ship::{ShipTractor, TractorState};
use crate::beam::{BEAM_DAMPING, BEAM_RANGE, BEAM_STIFFNESS, BEAM_STRENGTH, HOLD_OFFSET};
use avian2d::prelude::{Forces, LinearVelocity, ReadRigidBodyForces, WriteRigidBodyForces};
use bevy::math::Vec2;
use bevy::prelude::{Query, Transform, With, Without};

pub fn apply_tractor_force(
    lander: Query<(&Transform, &ShipTractor, &LinearVelocity), (With<Lander>, Without<Delivery>)>,
    mut packages: Query<(&Transform, Forces), (With<Delivery>, Without<Lander>)>,
) {
    let Ok((lander_t, ship_tractor, lander_v)) = lander.single() else {
        return;
    };
    if ship_tractor.state != TractorState::On {
        //no force unless fully ON
        return;
    }
    for (package_t, mut forces) in &mut packages {
        let hold = lander_t.translation.truncate() - Vec2::new(0.0, HOLD_OFFSET);
        let to_hold = hold - package_t.translation.truncate();
        let relative_v = forces.linear_velocity() - lander_v.0;
        //
        let force = (to_hold * BEAM_STIFFNESS - relative_v * BEAM_DAMPING)
            .clamp_length_max(BEAM_STRENGTH);

        let distance_to_lander = to_hold.length();
        if distance_to_lander > BEAM_RANGE || distance_to_lander < 1.0 {
            //too far
            continue;
        }

        forces.apply_force(force);

    }
}
