use crate::state::GameState;
use bevy::asset::{Asset, AssetServer};
use bevy::image::Image;
use bevy::prelude::{NextState, Res, ResMut, Resource, UntypedHandle};
use bevy_enoki::Particle2dEffect;
use crate::config;
use crate::particles::particles;

#[derive(Resource, Default)]
pub struct PendingAssets(pub Vec<UntypedHandle>);

fn push_asset<A: Asset>(server: &AssetServer, pending: &mut PendingAssets, asset_path: &str) {
    pending.0.push(server.load::<A>(asset_path.to_owned()).untyped());
}

pub fn start_preload(server: Res<AssetServer>, mut pending: ResMut<PendingAssets>) {
    for path in [
        config::sprites::SPRITE_PACKAGE,
        config::sprites::SPRITE_CITY,
        config::sprites::SPRITE_LANDER,
        config::sprites::SPRITE_ARROW,
        config::sprites::SPRITE_LASER,
        config::sprites::SPRITE_LIGHT,
        config::sprites::SP_TARGET_ARROW,
        config::sprites::SP_LANDING_PAD,
    ] {
        push_asset::<Image>(&server, &mut pending, path);
    }

    for path in [
        particles::PARTICLE_MAIN_THRUSTER,
        particles::PARTICLE_MAIN_THRUSTER_CORE,
        particles::PARTICLE_REVERSE_THRUSTER,
        particles::PARTICLE_LEFT_THRUSTER,
        particles::PARTICLE_RIGHT_THRUSTER,
        particles::PARTICLE_TRACTOR_BEAM,
        particles::PARTICLE_TRACTOR_BEAM_SPARKLE,
        particles::PARTICLE_EXPLOSION_FIRE,
        particles::PARTICLE_EXPLOSION_SPARKS,
    ] {
        push_asset::<Particle2dEffect>(&server, &mut pending, path);
    }
}

pub fn check_assets_loaded(
    server: Res<AssetServer>,
    pending: Res<PendingAssets>,
    mut next: ResMut<NextState<GameState>>,
) {
    if pending.0.iter()
        .all(|pend| server.is_loaded_with_dependencies(pend.id()))
    {
        next.set(GameState::Ready);
    }
}
