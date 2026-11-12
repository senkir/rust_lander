//TODO: eventually spawn world assets in here

use crate::world::layers::WorldLayer;
use crate::world::spawn;
use avian2d::parry::glamx::Vec2;
use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::color::Color;
use bevy::prelude::{Bundle, Commands, Res, Resource, Sprite, Transform, default};
use crate::theme::ThemeColors;
use crate::world::terrain::TerrainMarker;

pub const WORLD_HEIGHT: f32 = 2000.0;
pub const WORLD_WIDTH: f32 = 3000.0;

pub const WORLD_GRAVITY: f32 = 10.0;

pub const WORLD_DENSITY: f32 = 0.5;

#[derive(Resource)]
pub struct World {
    world_h: f32,
    world_w: f32,
    world_gravity: f32,
    world_density: f32,
}

impl Default for World {
    fn default() -> Self {
        Self {
            world_h: WORLD_HEIGHT,
            world_w: WORLD_WIDTH,
            world_gravity: WORLD_GRAVITY,
            world_density: WORLD_DENSITY,
        }
    }
}

impl World {

    pub fn width(&self) -> f32 {
        self.world_w
    }

    pub fn height(&self) -> f32 {
        self.world_h
    }
    pub fn left(self: &Self) -> f32 {
        -(self.world_w / 2.0)
    }

    pub fn right(self: &Self) -> f32 {
        (self.world_w / 2.0)
    }

    pub fn top(self: &Self) -> f32 {
        (self.world_h / 2.0)
    }

    pub fn bottom(self: &Self) -> f32 {
        -(self.world_h / 2.0)
    }
}

pub fn build_world(mut commands: Commands) {
    commands.insert_resource(World::default());
}

/// generate the environment from the world spec
pub fn spawn_world_bounds(mut commands: Commands, world: Res<World>, theme_colors: Res<ThemeColors>) {
    //TODO: figure out how to translate window dimensions to world dimensions

    let window_width = world.world_w;
    ///coodinates from origin 0,0
    let window_left = -(world.world_w / 2.0);
    let window_right = -window_left;
    let floor_thickness = 10.0;
    let window_ceiling = world.world_h / 2.0;
    let window_height = world.world_h;

    //TODO: spawn background

    //base floor
    commands.spawn(make_horizon(window_width, -(world.top() - floor_thickness), &theme_colors));

    // ceiling

    commands.spawn(make_horizon(window_width, (window_ceiling), &theme_colors));

    //base walls

    //left
    commands.spawn(make_wall(window_height, world.left(), &theme_colors));

    //right

    commands.spawn(make_wall(window_height, world.right(), &theme_colors));
}

fn make_wall(obj_height: f32, position_x: f32, theme_colors: &ThemeColors) -> impl Bundle {
    (
        Sprite {
            color: theme_colors.surface,
            custom_size: Some(Vec2::new(25.0, obj_height)),
            ..default()
        },
        Transform::from_xyz(position_x, 0.0, 0.0),
        RigidBody::Static,
        TerrainMarker,
        Collider::rectangle(10.0, obj_height),
        CollisionLayers::new(
            [WorldLayer::Floor],
            [
                WorldLayer::Floor,
                WorldLayer::Lander,
                WorldLayer::Package,
                WorldLayer::Enemies,
            ],
        ),
    )
}

fn make_horizon(width: f32, position_y: f32, theme_colors: &ThemeColors) -> impl Bundle {
    (
        Sprite {
            color: theme_colors.surface,
            custom_size: Some(Vec2::new(width, 25.0)),
            ..default()
        },
        Transform::from_xyz(0.0, (position_y), 0.0),
        RigidBody::Static,
        TerrainMarker,
        //this is the size of the collider itself at the matching position
        Collider::rectangle(width, 10.0),
        CollisionLayers::new(
            [WorldLayer::Floor],
            [
                WorldLayer::Floor,
                WorldLayer::Package,
                WorldLayer::Lander,
                WorldLayer::Enemies,
            ],
        ),
    )
}
