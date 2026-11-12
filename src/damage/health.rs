use bevy::prelude::{Component, MessageReader, ResMut};
use crate::lander::respawn::RespawnReadyMessage;

#[derive(Component, Copy, Clone, Debug)]
pub struct Health {
    pub current: u32,
    max: u32,
    dying: bool,
}

impl Default for Health {
    fn default() -> Health {
        Self {
            current: 100,
            max: 100,
            dying: false,
        }
    }
}

impl Health {
    
    pub fn new(max: u32) -> Self {
        Self {
            current: max,
            max,
            dying: false
        }
    }
    
    /// determine if health has reached threshold for death
    pub fn is_dead(&self) -> bool {
        self.current == 0
    }

    pub fn is_dying(&self) -> bool {
        self.dying
    }

    pub fn is_dead_not_dying(&self) -> bool {
        self.is_dead() && !self.dying
    }

    pub fn mark_for_death(&mut self) {
        self.dying = true;
    }

    pub fn apply_damage(&mut self, damage: u32) {
        self.current = self.current.saturating_sub(damage);
    }

    /// 0.0 (dead) to 1.0 (full)
    pub fn fraction(&self) -> f32 {
        if self.max == 0 { 0.0 } else { self.current as f32 / self.max as f32 }
    }
}