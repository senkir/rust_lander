use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{Component, Update};

pub mod camera;

#[derive(Component)]
pub struct CameraPlugin;


pub mod config {
    /// How fast the camera interpolates toward the player (higher = snappier)
    /// `lerp` = linear interpolation
    pub const CAMERA_LERP_SPEED: f32 = 6.0;
    /// Z position for the camera (must be high to see all layers)
    pub const CAMERA_Z: f32 = 1000.0; //big boi number
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera::create_camera)
            .add_systems(Update, (camera::follow_camera,
            camera::apply_parallax));
    }
}
