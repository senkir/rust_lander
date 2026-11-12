pub mod game_state;
pub mod loading;
pub mod pause;
pub mod preload;
pub mod death;
mod start;
pub mod button;

use bevy::prelude::*;

use crate::{damage, deliveries, delivery_target, lander, score};
use crate::world;
pub use game_state::GameState;
use crate::lander::assets;
use crate::lander::assets::SpriteAtlasHandles;
use crate::state::preload::PendingAssets;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PendingAssets>();
        app.init_resource::<SpriteAtlasHandles>();

        app.init_state::<GameState>()
            // Loading state systems
            .add_systems(
                OnEnter(GameState::Loading),
                //everything that happens in loading
                (
                    world::spawn::build_world,
                    world::terrain::build_terrain,
                    assets::preload_sprite_maps,
                    loading::spawn_loading_screen,
                    preload::start_preload
                ).chain(),
            )
            .add_systems(
                Update,
                (preload::check_assets_loaded,
                 loading::animate_loading)
                    .run_if(in_state(GameState::Loading)),
            )
            .add_systems(
                OnEnter(GameState::Ready),
                (start::spawn_ready_screen,
                pause::pause_physics)
            )
            .add_systems(
                OnExit(GameState::Ready),
                pause::resume_physics
            )
            .add_observer(
                button::on_button_out_style);
            app.add_observer(button::on_button_over_style);
            app.add_systems(
                OnExit(GameState::Loading),
                (
                    world::spawn::spawn_world_bounds,
                    world::terrain::spawn_terrain_entities,
                    world::terrain::spawn_background,
                    loading::despawn_loading_screen,
                    lander::spawn::spawn_lander,
                    deliveries::spawn::spawn_package,
                    delivery_target::spawn::spawn_target,
                    score::display::spawn_score,
                    damage::bar::spawn_lander_health_bar
                ),
            )
            // Pause state systems
            .add_systems(OnEnter(GameState::Paused), (
                pause::spawn_pause_menu, pause::pause_physics))
            .add_systems(OnExit(GameState::Paused),
                         (pause::despawn_pause_menu, pause::resume_physics))
            // Pause toggle (works in Playing or Paused states)
            .add_systems(
                Update,
                toggle_pause
                    .run_if(in_state(GameState::Playing).or_else(in_state(GameState::Paused))),
            )
            .add_systems(
                OnEnter(GameState::Dead),(
                    death::spawn_death_screen,
                    pause::pause_physics
                    )
            ).add_systems(
            OnExit(GameState::Dead), (
                death::despawn_death_screen,
                pause::resume_physics
            )
        )
        ;
    }
}

fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => {
                info!("Game paused");
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                info!("Game resumed");
                next_state.set(GameState::Playing);
            }
            _ => {}
        }
    }
}
