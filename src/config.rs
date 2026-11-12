
pub mod sprites {
    pub const SPRITE_LANDER: &str = "lander_sprite.png";

    pub const SPRITE_PACKAGE: &str = "cow_sprite.png";
    pub const SPRITE_CITY: &str = "bg_city.png";

    pub const SPRITE_LASER: &str = "noun_laser.png";
    pub const SPRITE_ARROW: &str = "noun_arrow.png";
    pub const SPRITE_LIGHT: &str = "noun_light.png";
    pub const SP_LANDING_PAD: &str = "landing_pad.png";
    pub const SP_TARGET_ARROW: &str = "target_arrow.png";
}
pub mod world {
    pub const GRAVITY: f32 = 100.0;

    pub const ANGULAR_DRAG: f32 = 1.5;
    pub const LINEAR_DRAG: f32 = 0.5;

    /// pixels which will be used to indicate 1m in reference size
    pub const WORLD_SIZE: f32 = 50.0;

    pub const WORLD_SEGMENTS: usize = 5;

}

pub mod hud {
    pub const HEALTH_POSITION_Y : f32 = 10.0;
    pub const SCORE_POSITION_Y : f32 = 30.0;
    pub const TIMER_POSITION_Y : f32 = 60.0;
}

pub mod lander {

    pub const MASS: f32 = 500.0;
    pub const LANDER_SIZE: f32 = 50.0;

    pub const LANDER_HEALTH: u32 = 50;

    pub const LANDER_START_X: f32 = 0.0;
    pub const LANDER_START_Y: f32 = 0.0;

    pub const LANDER_Z_POS: f32 = 10.0;

    pub const THRUSTER_FORCE: f32 = 500.0;

    pub const THRUSTER_FORCE_SIDE: f32 = 50.0;
    pub const THRUSTER_FORCE_REVERSE: f32 = 100.0;

    pub const ROTATION_FORCE: f32 = 10.0;
}

pub mod thruster {
    pub const SIDE_THRUSTER_Y_OFFSET: f32 = -5.0;
    pub const SIDE_THRUSTER_X_OFFSET: f32 = 20.0;
    pub const THRUSTER_Y_OFFSET: f32 = -10.0;
    pub const MAIN_THRUSTER_RADIUS: f32 = 50.0;
    pub const SIDE_THRUSTER_RADIUS: f32 = 20.0;
}