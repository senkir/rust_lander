use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Velocity(pub Vec2);

impl Velocity {
    
    pub const ZERO: Self = Self(Vec2::ZERO);
    
    /// is the target moving
    pub fn is_moving(&self) -> bool {
        self.0 != Vec2::ZERO
    }
}