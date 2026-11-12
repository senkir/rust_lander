use avian2d::parry::glamx::Vec2;

pub(crate) fn calculate_thrust_vector(thruster_force: f32, rotation_radians: f32) -> Vec2 {
    //TODO: run this calculation
    Vec2::new(-rotation_radians.sin(), rotation_radians.cos()) * thruster_force
}
