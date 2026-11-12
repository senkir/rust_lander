use bevy::prelude::{Component, MessageReader, ResMut};
use crate::lander::respawn::RespawnReadyMessage;

#[derive(Component, Copy, Clone, Debug)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Default for Health {
    fn default() -> Health {
        Self {
            current: 100,
            max: 100,
        }
    }
}

impl Health {
    
    pub fn new(max: u32) -> Self {
        Self {
            current: max,
            max
        }
    }
    
    /// determine if health has reached threshold for death
    pub fn is_dead(&self) -> bool {
        self.current == 0
    }
    
    pub fn apply_damage(&mut self, damage: u32) {
        self.current = self.current.saturating_sub(damage);
    }

    /// 0.0 (dead) to 1.0 (full)
    pub fn fraction(&self) -> f32 {
        if self.max == 0 { 0.0 } else { self.current as f32 / self.max as f32 }
    }
}

pub fn damage_stage(health: &Health) -> usize {
    match health.fraction() {
        f if f > 0.75 => 0, //healthy
        f if f > 0.5 => 1, //damaged
        f if f > 0.3 => 2, //damaged
        _ => 3, //critical
    }
}