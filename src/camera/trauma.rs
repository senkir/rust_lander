use bevy::prelude::{Component, Resource, Vec2};

#[derive(Resource, Default)]
pub struct CameraShake {
    pub trauma: f32
}

#[derive(Component)]
pub struct ShakeOffset {
    pub offset: Vec2
}

impl Default for ShakeOffset {
    fn default() -> Self {
        ShakeOffset { offset: Vec2::ZERO }
    }
}

pub const MAX_OFFSET: f32 = 24.0;
pub const MAX_ROLL: f32 = 0.04;
pub const DECAY: f32 = 1.4;