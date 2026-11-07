
//TODO: eventually spawn world assets in here

use avian2d::parry::glamx::Vec2;
use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::ops::floor;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::{default, Circle, ColorMaterial, Commands, MeshMaterial2d, ResMut, Sprite, Transform, Query, Window};
use bevy::window::WindowResolution;
use crate::config;
use crate::world::layers::WorldLayer;

pub fn spawn_world(mut commands: Commands,
                   mut materials: ResMut<Assets<ColorMaterial>>,
                   mut meshes: ResMut<Assets<Mesh>>,
                   query: Query<&Window>,
) {

    let Ok(window) = query.single() else {
        return;
    };

    let window_width = window.width();
    ///coodinates from origin 0,0
    let window_left = -(window.width() / 2.0);
    let window_right = -window_left;
    let floor_thickness = 10.0;
    let window_floor = window.height() / 2.0;
    let window_height = window.height();
    let window_top = window.height() / 2.0;

    //TODO: spawn background

    //base floor
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.7, 0.9),
            custom_size: Some(Vec2::new(window_width, 25.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -(window_floor - floor_thickness), 0.0),
        RigidBody::Static,
        //this is the size of the collider itself at the matching position
        Collider::rectangle(window_width, floor_thickness),
        CollisionLayers::new(
            [WorldLayer::Floor],
            [WorldLayer::Floor, WorldLayer::Package, WorldLayer::Lander, WorldLayer::Enemies]),
    ));

    //base walls

    //left
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.7, 0.9),
            custom_size: Some(Vec2::new(25.0, window_height)),
            ..default()
        },
        //global shift
        // TC: sprites seem to span where Y = 0
        Transform::from_xyz(window_left, 0.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(10.0, window_height),
        CollisionLayers::new(
            [WorldLayer::Floor],
            [WorldLayer::Floor, WorldLayer::Package, WorldLayer::Lander, WorldLayer::Enemies]),
    ));

    //right

    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.7, 0.9),
            custom_size: Some(Vec2::new(25.0, window_height)),
            ..default()
        },
        Transform::from_xyz(window_right, 0.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(10.0, window_height),
        CollisionLayers::new(
            [WorldLayer::Floor],
            [WorldLayer::Floor, WorldLayer::Lander, WorldLayer::Package, WorldLayer::Enemies]),
    ));
}


pub fn gen_vectors(world_width: f32, world_height: f32)  {
    // let segments = config::world::WORLD_SEGMENTS;

    //TODO: not implemented

    // Collider::compound()
}