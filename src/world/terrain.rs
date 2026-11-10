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
const PAD_WIDTH_STEPS: f32 = 6.0;              // pad is 6 * STEP = 120 px wide
const PAD_BLEND: f32 = 5.0 * STEP;             // ramp length either side of the pad

#[derive(Resource)]
pub struct Terrain {
    pub points: Vec<Vec2>,
}

pub struct Pad { pub x: f32, pub y: f32, pub width: f32 }

fn raw_height(x: f32, seed: f32) -> f32 {
    (x * 0.004 + seed).sin() * 60.0
        + (x * 0.011 + seed * 2.3).sin() * 25.0
        + (x * 0.027 + seed * 4.1).sin() * 8.0
}
fn height_at(x: f32, seed: f32, pads: &[Pad]) -> f32 {
    let raw = BASE_Y + raw_height(x, seed);
    for pad in pads {
        let d = (x - pad.x).abs();
        let half = pad.width / 2.0;
        if d <= half {
            return pad.y;                                  // exactly flat under the pad
        } else if d < half + PAD_BLEND {
            let t = ((d - half) / PAD_BLEND).clamp(0.0, 1.0);
            let t = t * t * (3.0 - 2.0 * t);               // smoothstep
            return pad.y + (raw - pad.y) * t;              // ease back into the curve
        }
    }
    raw
}


fn hash01(n: f32) -> f32 { (n.sin() * 43758.547).fract().abs() }

// fn plan_pads(world_w: f32, count: usize, seed: f32) -> Vec<Pad> {
//     let zone = world_w / count as f32;
//     (0..count).map(|i| {
//         let zone_start = -world_w / 2.0 + i as f32 * zone;
//         // keep the pad in the middle 60 percent of its zone, so neighbors never overlap
//         let raw_x = zone_start + zone * (0.2 + 0.6 * hash01(seed + i as f32 * 12.9898));
//         let x = (raw_x / STEP).round() * STEP;                       // snap to the grid
//         let y = BASE_Y + raw_height(x, seed);                        // height of the unmodified curve
//         Pad { x, y, width: PAD_WIDTH_STEPS * STEP }
//     }).collect()
// }

/// generate a terrain floor
pub fn spawn_terrain(mut commands: Commands, world: Res<spawn::World>) {
    let half_w = world.world_w / 2.0;

    // sample the surface across the whole window
    let count = (world.world_w / STEP).ceil() as usize + 1;
    let points: Vec<Vec2> = (0..count)
        .map(|i| {
            let x = -half_w + i as f32 * STEP;
            Vec2::new(x, BASE_Y + raw_height(x, SEED))
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
