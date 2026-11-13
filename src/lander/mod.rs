use crate::state::GameState;
use crate::thruster::{main_thruster, secondary_thruster};
use bevy::app::App;
use bevy::prelude::{Component, IntoScheduleConfigs, Plugin, Update, in_state};
use crate::lander::audio::EngineMessage;
use crate::lander::respawn::{RespawnReadyMessage, RespawnBeginMessage};

pub mod debug;
pub mod input;
pub mod lander;
pub mod ship;
pub mod spawn;
pub mod assets;
pub mod respawn;
pub mod audio;

#[derive(Component)]
pub struct LanderPlugin;

impl Plugin for LanderPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<EngineMessage>();
        app.add_message::<RespawnBeginMessage>();
        app.add_message::<RespawnReadyMessage>();
        app.add_systems(Update, (debug::debug_reset_sprite))
            .add_systems(
                Update,
                (
                    input::observe_thruster_pressed,
                    audio::add_engine_sound,
                    (spawn::observe_respawn_ready,
                    spawn::observe_respawn_start).chain(),
                    main_thruster::handle_thrust_active_state,
                    secondary_thruster::handle_rotation_thrust,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
