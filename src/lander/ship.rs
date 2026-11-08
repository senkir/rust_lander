use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct ShipStatus {
    pub is_thrusting: bool,
    pub is_rotating: bool,
    pub angular_force: f32,
    pub velocity: f32,
}
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
