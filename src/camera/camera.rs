use bevy::camera::{Camera, Camera2d, ClearColorConfig};
use bevy::color::Color;
use bevy::core_pipeline::tonemapping::DebandDither;
use bevy::log::error;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::{default, Changed, Commands, Component, Query, Res, Time, Transform, With, Without};
use crate::camera::config;
use crate::lander::lander::Lander;

/// Marker component for the main game camera.
#[derive(Component)]
pub struct MainCamera;


pub fn create_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        MainCamera,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        // Tonemapping::TonyMcMapface,
        DebandDither::Enabled,
        Bloom::default(),
    ));
}


pub fn follow_camera(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Lander>, Changed<Transform>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Lander>)>,
) {
    // The Changed<Transform> filter in player_query means this system only processes
    // when the player has moved - Bevy filters it at the query level
    let Some(player_transform) = player_query.iter().next() else {
        return;
    };

    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    let camera_pos = camera_transform.translation.truncate();

    // Early exit if camera is already very close (within 0.5 pixels)
    let distance = player_pos.distance(camera_pos);
    if distance < 0.5 {
        error!("Camera is too far!");
        return;
    }

    // Smooth interpolation toward player
    let lerp_factor = (config::CAMERA_LERP_SPEED * time.delta_secs()).clamp(0.0, 1.0);
    let new_pos = camera_pos.lerp(player_pos, lerp_factor);

    // Snap to pixel boundaries to prevent grid shimmer
    camera_transform.translation.x = new_pos.x.round();
    camera_transform.translation.y = new_pos.y.round();
    camera_transform.translation.z = config::CAMERA_Z;
}