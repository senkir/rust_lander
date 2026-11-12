use std::ops::Sub;
use avian2d::parry::glamx::Vec2;
use bevy::camera::{Camera, Camera2d, ClearColorConfig};
use bevy::color::Color;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::log::error;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::{default, Changed, Commands, Component, Query, Res, Time, Transform, With, Without, ResMut, Local};
use crate::camera::{config, trauma};
use crate::camera::trauma::{CameraShake, ShakeOffset};
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
        Tonemapping::TonyMcMapface,
        DebandDither::Enabled,
        Bloom::default(),
        ShakeOffset::default(),
    ));
}

pub fn apply_shake(
    time: Res<Time>,
    mut shake: ResMut<CameraShake>,
    mut cam: Query<(&mut Transform, &mut ShakeOffset), With<MainCamera>>,
) {
    let Ok((mut transform, mut shake_offset)) = cam.single_mut() else {
        return;
    };

    if (shake.trauma == 0.0) {
        return;
    }

    transform.translation -= shake_offset.offset.extend(0.0);

    //this is supposed to reduce over time
    shake.trauma = (shake.trauma - trauma::DECAY * time.delta_secs()).max(0.0);
    let amp = shake.trauma * shake.trauma;
    let tt = time.delta_secs() * 40.0; //frequency

    let offset = Vec2::new(
        (tt * 1.00).sin() + (tt * 2.30).sin() * 0.5,
        (tt * 1.27 + 1.0).sin() + (tt * 2.71).sin() * 0.5,
    ) * (amp * trauma::MAX_OFFSET / 1.5);

    transform.translation = offset.extend(0.0);
    shake_offset.offset = offset;
}


pub fn follow_camera(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Lander>, Changed<Transform>)>,
    mut camera_query: Query<(&mut Transform, &ShakeOffset), (With<MainCamera>, Without<Lander>)>,
) {
    // The Changed<Transform> filter in player_query means this system only processes
    // when the player has moved - Bevy filters it at the query level
    let Some(player_transform) = player_query.iter().next() else {
        return;
    };

    let Ok((mut camera_transform, shake_offset)) = camera_query.single_mut() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    //shift back from the shake position
    camera_transform.translation -= shake_offset.offset.extend(0.0);
    let camera_pos = camera_transform.translation.truncate();

    // Early exit if camera is already very close (within 0.5 pixels)
    let distance = player_pos.distance(camera_pos);
    if distance < 0.5 {
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

#[derive(Component)]
pub struct Parallax {
    pub factor: Vec2, // 0.0 = fixed in the world, 1.0 = fixed on screen
}


pub fn apply_parallax(
    camera: Query<&Transform, (With<MainCamera>, Changed<Transform>)>,
    mut layers: Query<(&mut Transform, &Parallax), Without<MainCamera>>,
) {
    let Ok(cam) = camera.single() else { return; };
    for (mut t, p) in &mut layers {
        t.translation.x = cam.translation.x * p.factor.x;
        t.translation.y = cam.translation.y * p.factor.y;
    }
}