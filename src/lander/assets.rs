use crate::config::sprites;
use bevy::asset::Handle;
use bevy::image::{Image, TextureAtlas};
use bevy::prelude::{
    AssetServer, Assets, Res, ResMut, Resource, Sprite, TextureAtlasLayout, UVec2,
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct TileMapHandles {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl TileMapHandles {
    pub fn sprite(&self, atlas_index: usize) -> Sprite {
        Sprite::from_atlas_image(
            self.image.clone(),
            TextureAtlas::from(self.layout.clone()).with_index(atlas_index),
        )
    }
}

#[derive(Resource, Clone, Default)]
pub struct SpriteAtlasHandles {
    map: HashMap<&'static str, TileMapHandles>,
}

impl SpriteAtlasHandles {

    pub fn add(&mut self, key: &'static str, sprite: TileMapHandles) {
        self.map.insert(key, sprite);
    }
    pub fn get(&self, name: &str) -> Option<&TileMapHandles> {
        self.map.get(name)
    }
}

pub fn preload_sprite_maps(
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<SpriteAtlasHandles>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    preload_texture_map(
        &asset_server,
        &mut atlases,
        &mut texture_atlas_layouts,
        sprites::SPRITE_LANDER,
        64,
        4,
        1,
    );

    preload_texture_map(
        &asset_server,
        &mut atlases,
        &mut texture_atlas_layouts,
        sprites::SPRITE_PACKAGE,
        64,
        14,
        1,
    );

    preload_texture_map(
        &asset_server,
        &mut atlases,
        &mut texture_atlas_layouts,
        sprites::SP_TARGET_ARROW,
        32,
        9,
        1
    )
}

fn preload_texture_map(
    asset_server: &AssetServer,
    atlases: &mut SpriteAtlasHandles,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    path: &'static str,
    tile_size: u32,
    columns: u32,
    rows: u32,
) {
    let texture: Handle<Image> = asset_server.load(path);

    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(tile_size), columns, rows, None, None);

    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let handle = TileMapHandles {
        image: texture,
        layout: texture_atlas_layout,
    };
    atlases.map.insert(path, handle);
}
