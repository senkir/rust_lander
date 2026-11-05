use avian2d::parry::glamx::Vec3;
use bevy::color::Color;
use bevy::prelude::Component;

#[derive(Component, Default, Clone, Copy, Eq, PartialEq)]
pub enum ThrusterType {
    #[default]
    Primary,
    Secondary
}

#[derive(Copy, Clone)]
pub struct ThrusterVisuals {
    pub particle_per_spawn: u32,
    
}

#[derive(Clone)]
pub struct ParticleConfig {
    //how long it exits (sec)
    pub lifetime: f32,
    pub lifetime_variance: f32,
    /// initial velocity
    pub speed: f32,
    /// variability
    pub speed_variance: f32,
    /// which way
    pub direction: Vec3,
    /// direction in `rads`
    pub direction_variance: f32,  // In radians
    /// size of particles
    pub scale: f32,
    pub scale_variance: f32,
    /// base tint
    pub color: Color,
    /// how fast particle spins
    pub angular_velocity: f32,
    pub angular_velocity_variance: f32,
    /// constant force applied
    pub acceleration: Vec3,
}

impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            lifetime: 1.0,
            lifetime_variance: 0.1,
            speed: 100.0,
            speed_variance: 10.0,
            direction: Vec3::X,
            direction_variance: 0.1,
            scale: 1.0,
            scale_variance: 0.1,
            color: Color::WHITE,
            angular_velocity: 0.0,
            angular_velocity_variance: 0.0,
            acceleration: Vec3::ZERO,
        }
    }
}
