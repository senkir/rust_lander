use crate::anim::{AnimationIndices, AnimationMode, AnimationTimer};
use crate::config::sprites;
use crate::lander::assets::SpriteAtlasHandles;
use crate::world::layers::WorldLayer;
use crate::world::terrain::Terrain;
use avian2d::parry::glamx::Vec2;
use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::math::Vec3;
use bevy::prelude::{children, default, AssetServer, Bundle, Commands, Component, Node, Res, Sprite, Transform, Val};
use bevy::time::TimerMode;
use rand::RngExt;

pub const PLATFORM_SIZE: f32 = 150.0;
#[derive(Component)]
pub struct LandingPad;

pub fn largest_y(points: [Vec2; 3]) -> f32 {
    let mut max: f32 = 0.0;
    for point in points {
        max = max.max(point.y);
    }
    max
}
pub fn spawn_target(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    sprite_atlas_handles: Res<SpriteAtlasHandles>,
    terrain: Res<Terrain>,
) {
    let points_size: u32 = terrain.size();

    let mut rng = rand::rng();
    let point_x = rng.random_range(1..points_size - 1) as f32;

    let final_y = terrain.y_at_x(point_x);

    //TODO: figure out floor at a position X
    commands.spawn((
        LandingPad,
        Sprite {
            image: asset_server.load(sprites::SP_LANDING_PAD),
            // color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(PLATFORM_SIZE, PLATFORM_SIZE)),
            ..default()
        },
        //figure out the position
        //little bit above the line position
        Transform::from_xyz(point_x, (final_y+50.0), 2.0),
        RigidBody::Static,
        //this is the size of the collider itself at the matching position

        //TODO: this is a little big
        Collider::rectangle(PLATFORM_SIZE, 30.0),
        CollisionLayers::new(
            [WorldLayer::Target],
            [
                WorldLayer::Floor,
                WorldLayer::Package,
                WorldLayer::Lander,
                WorldLayer::Enemies,
            ],
        ),
        children![landing_pad_arrow(&sprite_atlas_handles)],
    ));
}

#[derive(Component)]
pub struct BouncingArrow;

fn landing_pad_arrow(sprite_atlas_handles: &SpriteAtlasHandles) -> impl Bundle {
    let bouncing_arrow = sprite_atlas_handles.get(sprites::SP_TARGET_ARROW);
    (
        bouncing_arrow.unwrap().sprite(0),
        BouncingArrow,
        AnimationIndices::new(0, 8, AnimationMode::Repeating),
        AnimationTimer::from_seconds(0.1),
        Transform::from_xyz(
            0.0, 60.00, 4.0, //TODO: z indez
        ).with_scale(Vec3::splat(1.5)),
    )
}
