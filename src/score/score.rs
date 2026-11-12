use bevy::prelude::{Commands, Resource};

/// game overall score
#[derive(Resource, Copy, Clone, Debug)]
pub struct Score {
    pub score: u32,
    pub packages: u32
}

impl Default for Score {
    fn default() -> Self {
        Self {
            score: 0,
            packages: 0
        }
    }
}
