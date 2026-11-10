use crate::world::layers::WorldLayer;
use crate::world::spawn;
use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Gizmos, Res, Resource, Transform};

/// horizontal spacing of sample points
const STEP: f32 = 20.0;
/// average ground height (world coordinates)
const BASE_Y: f32 = -250.0;
/// how far the solid body extends below the surface
const DEPTH: f32 = 200.0;
const SEED: f32 = 1.7;

#[derive(Resource)]
pub struct Terrain {
    pub points: Vec<Vec2>,
}

fn height_at(x: f32, seed: f32) -> f32 {
    (x * 0.004 + seed).sin() * 60.0
        + (x * 0.011 + seed * 2.3).sin() * 25.0
        + (x * 0.027 + seed * 4.1).sin() * 8.0
}

/// generate a terrain floor
pub fn spawn_terrain(mut commands: Commands, world: Res<spawn::World>) {
    //TODO: replace the window size with something scaling to a world width
    let half_w = world.world_w / 2.0;

    // sample the surface across the whole window
    let count = (world.world_w / STEP).ceil() as usize + 1;
    let points: Vec<Vec2> = (0..count)
        .map(|i| {
            let x = -half_w + i as f32 * STEP;
            Vec2::new(x, BASE_Y + height_at(x, SEED))
        })
        .collect();

    // one convex trapezoid per segment, combined into a single compound collider
    let shapes: Vec<(Vec2, f32, Collider)> = points
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

    commands.insert_resource(Terrain { points });
}

pub fn draw_terrain(mut gizmos: Gizmos, terrain: Option<Res<Terrain>>) {
    let Some(terrain) = terrain else {
        return;
    };
    gizmos.linestrip_2d(terrain.points.iter().copied(), Color::srgb(0.2, 0.7, 0.9));
}
