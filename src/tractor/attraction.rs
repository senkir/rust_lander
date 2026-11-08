use crate::deliveries::delivery::Delivery;
use crate::lander::lander::Lander;
use super::ship::{ShipTractor, TractorState};
use crate::tractor::{BEAM_DAMPING, BEAM_RANGE, BEAM_STRENGTH};
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
    if ship_tractor.state == TractorState::Off {
        //no force if not ON
        return;
    }
    for (package_t, mut forces) in &mut packages {
        let to_lander = (lander_t.translation - package_t.translation).truncate();
        let distance_to_lander = to_lander.length();
        if distance_to_lander > BEAM_RANGE || distance_to_lander < 1.0 {
            //too far
            continue;
        }

        //TODO: pull towards a little under the lander
        //TODO: counter gravity a little so package can lift

        //TODO: what is this
        let direction = to_lander / distance_to_lander;

        let falloff = 1.0 - distance_to_lander / BEAM_RANGE;

        //damp velocity relative to lander so it doesn't orbit or oscillate
        let relative_v = forces.linear_velocity() - lander_v.0;
        forces.apply_force(direction * BEAM_STRENGTH * falloff - relative_v * BEAM_DAMPING);
    }
}
