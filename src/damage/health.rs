use bevy::prelude::Component;

#[derive(Component, Copy, Clone, Debug)]
pub struct Health(u32);

impl Health {
    
    pub fn new(val: u32) -> Self {
        Health(val)
    }
    
    /// determine if health has reached threshold for death
    pub fn is_dead(&self) -> bool {
        self.0 == 0
    }
    
    pub fn apply_damage(&mut self, damage: u32) {
        self.0 = self.0.saturating_sub(damage);
    }
}