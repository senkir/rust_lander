use std::cmp::min;
use crate::camera::camera::{MainCamera, Parallax};
use crate::config::sprites;
use crate::world::layers::WorldLayer;
use crate::world::spawn;
use crate::world::spawn::World;
use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::asset::Handle;
use bevy::color::Color;
use bevy::image::Image;
use bevy::math::Vec2;
use bevy::prelude::{AssetServer, Changed, Commands, Component, Entity, Gizmos, Query, Res, Resource, Transform, Window, With, Without};
use bevy::sprite::Sprite;
use bevy::window::PrimaryWindow;
use rand::RngExt;

/// horizontal spacing of sample points
const STEP: f32 = 20.0;
/// average ground height (world coordinates)
const BASE_Y: f32 = -250.0;
/// how far the solid body extends below the surface
const DEPTH: f32 = 200.0;
const SEED: f32 = 1.7;
const PAD_WIDTH_STEPS: f32 = 6.0; // pad is 6 * STEP = 120 px wide
const PAD_BLEND: f32 = 5.0 * STEP; // ramp length either side of the pad

#[derive(Resource)]
pub struct Terrain {
    pub points: Vec<Vec2>,
}

pub struct Pad {
    pub x: f32,
    pub y: f32,
    pub width: f32,
}

fn raw_height(x: f32, seed: f32) -> f32 {
    (x * 0.004 + seed).sin() * 60.0
        + (x * 0.011 + seed * 2.3).sin() * 25.0
        + (x * 0.027 + seed * 4.1).sin() * 8.0
}

pub fn spawn_background(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    world: Res<World>,
    camera: Query<Entity, With<MainCamera>>,
) {
    let Ok(camera) = camera.single() else {
        return;
    };

    let city: Handle<Image> = asset_server.load(sprites::SPRITE_CITY);

    let window = windows.single();

    let un = window.unwrap();
    let width = un.width();
    let height = un.height();

    commands.spawn((
        Sprite {
            image: city,
            custom_size: Some(Vec2::new(width,width)),
            // custom_size: Some(Vec2::new(world.width(), world.height())),
            ..Default::default()
        },
        //TODO: fix this
        Parallax {
           factor: Vec2::new(0.3, 0.1),
        },
        Transform::from_xyz(0.0,0.0, 0.0), // behind the world, in front of the far plane
        // Transform::from_xyz(world.left(), world.top(), -10.0), // behind the world, in front of the far plane
    ));
}

pub fn spawn_terrain_entities(mut commands: Commands, terrain: Res<Terrain>) {
    // one convex trapezoid per segment, combined into a single compound collider
    let shapes: Vec<(Vec2, f32, Collider)> = terrain
        .points
        .windows(2)
        .filter_map(|w| {
            let (a, b) = (w[0], w[1]);
            let bottom = a.y.min(b.y) - DEPTH;
            Collider::convex_hull(&[a, b, Vec2::new(b.x, bottom), Vec2::new(a.x, bottom)])
                .map(|hull| (Vec2::ZERO, 0.0, hull))
        })
        .collect();

    commands.spawn((
        Transform::default(),
        RigidBody::Static,
        Collider::compound(shapes),
        CollisionLayers::new(
            [WorldLayer::Floor],
            [
                WorldLayer::Floor,
                WorldLayer::Package,
                WorldLayer::Lander,
                WorldLayer::Enemies,
            ],
        ),
    ));
}

/// generate a terrain floor
pub fn build_terrain(mut commands: Commands, world: Res<spawn::World>) {
    let half_w = world.width() / 2.0;

    // sample the surface across the whole window
    let count = (world.width() / STEP).ceil() as usize + 1;

    //randomize seed value

    let mut rng = rand::rng();
            let seed=rng.random_range(2.5..10.0);
    let points: Vec<Vec2> = (0..count)
        .map(|i| {
            let x = -half_w + i as f32 * STEP;


            Vec2::new(x, BASE_Y + raw_height(x, seed))
        })
        .collect();

    commands.insert_resource(Terrain { points });
}

pub fn draw_terrain(mut gizmos: Gizmos, terrain: Option<Res<Terrain>>) {
    let Some(terrain) = terrain else {
        return;
    };
    //TODO: change from a gizmo to an asset
    gizmos.linestrip_2d(terrain.points.iter().copied(), Color::srgb(0.2, 0.7, 0.9));
}
