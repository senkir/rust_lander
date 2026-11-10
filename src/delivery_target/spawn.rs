use avian2d::parry::glamx::Vec2;
use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::color::Color;
use bevy::prelude::{default, Commands, Sprite, Transform, Component, Res};
use rand::RngExt;
use crate::world::layers::WorldLayer;
use crate::world::terrain::Terrain;

pub const PLATFORM_WIDTH: f32 = 100.0;
#[derive(Component)]
pub struct LandingPad;

pub fn largest_y(points: [Vec2; 3]) -> f32 {
    let mut max: f32 = 0.0;
    for point in  points {
        max = max.max(point.y);
    }
    max
}
pub fn spawn_target(
    mut commands: Commands,
    terrain: Res<Terrain>,
) {

    let points_size = terrain.points.len();

    let mut rng = rand::rng();
    let get_val = rng.random_range(1..points_size-1);

    let vec_at = terrain.points[get_val];
    let vec_after = terrain.points[get_val+1];
    let vec_before = terrain.points[get_val-1];
    // let final_y = largest_y([vec_at, vec_after, vec_before]);
    let final_y = vec_at.y;

    let obj_width = PLATFORM_WIDTH;
    let obj_height = 10.0;
    //TODO: figure out floor at a position X
    commands.spawn((
    Sprite {
        color: Color::srgb(0.8, 0.2, 0.2),
        custom_size: Some(Vec2::new(obj_width, obj_height)),
        ..default()
    },
    //figure out the position
        //little bit above the line position
    Transform::from_xyz(vec_at.x, (final_y + obj_height), 2.0),
    RigidBody::Static,
    //this is the size of the collider itself at the matching position
    Collider::rectangle(obj_width, obj_height),
    CollisionLayers::new(
        [WorldLayer::Target],
        [
            WorldLayer::Floor,
            WorldLayer::Package,
            WorldLayer::Lander,
            WorldLayer::Enemies,
        ],
    ),
    ));
    //
    //
    // for pad in &pads {
    //     let thickness = 8.0;
    //     commands.spawn((
    //         LandingPad,
    //         Sprite {
    //             color: Color::srgb(0.9, 0.8, 0.2),
    //             custom_size: Some(Vec2::new(pad.width, thickness)),
    //             ..default()
    //         },
    //         // the top surface sits exactly at pad.y
    //         Transform::from_xyz(pad.x, pad.y - thickness / 2.0, 0.1),
    //         RigidBody::Static,
    //         Collider::rectangle(pad.width, thickness),
    //         CollisionLayers::new(
    //             [WorldLayer::Floor],
    //             [WorldLayer::Floor, WorldLayer::Package, WorldLayer::Lander, WorldLayer::Enemies],
    //         ),
    //     ));
    // }
    // commands.insert_resource(Pads(pads));
}