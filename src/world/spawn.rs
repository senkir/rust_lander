
//TODO: eventually spawn world assets in here

use avian2d::parry::glamx::Vec2;
use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::ops::floor;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::{default, Circle, ColorMaterial, Commands, MeshMaterial2d, ResMut, Sprite, Transform, Query, Window};
use bevy::window::WindowResolution;
use crate::world::layers::WorldLayer;

pub fn spawn_world(mut commands: Commands,
                   mut materials: ResMut<Assets<ColorMaterial>>,
                   mut meshes: ResMut<Assets<Mesh>>,
                   query: Query<&Window>,
) {

    let Ok(window) = query.single() else {
        return;
    };

    let window_x = window.width();
    let floor_thickness = 10.0;
    let window_y = window.height() / 2.0 - floor_thickness;

    //TODO: spawn background

    //TODO: generate floor
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.7, 0.9),
            custom_size: Some(Vec2::new(500.0, 25.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -window_y, 0.0),
        RigidBody::Static,
        Collider::rectangle(window_x, floor_thickness),
        CollisionLayers::new(
            [WorldLayer::Floor],
            [WorldLayer::Floor, WorldLayer::Lander, WorldLayer::Enemies]),
    ));
}
