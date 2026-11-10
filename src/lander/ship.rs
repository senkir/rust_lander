use std::fmt::Display;
use bevy::prelude::Component;
use crate::lander::input::InputDirection;

#[derive(Component, Debug)]
pub struct ShipStatus {
    pub thrust_direction: InputDirection,
    pub angular_force: f32,
    pub velocity: f32,
}

impl Display for ShipStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "is_thrusting {}, is_rotating {} rotation_direction {}",
               self.is_thrusting(), self.use_secondary_thrust(), self.thrust_direction)
    }
}
impl ShipStatus {
    pub fn use_secondary_thrust(&self) -> bool {
        self.thrust_direction == InputDirection::Right || self.thrust_direction == InputDirection::Left
    }
    pub fn is_thrusting(&self) -> bool {
        self.thrust_direction == InputDirection::Forward || self.thrust_direction == InputDirection::Backward
    }

    pub fn new() -> Self {
        Self {
            angular_force: 0.0,
            thrust_direction: InputDirection::None,
            velocity: 0.0,
        }
    }
}
