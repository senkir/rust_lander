use bevy::prelude::States;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    //TODO: show a splash screen
    Splash,
    #[default]
    Loading,
    Ready,
    Playing,
    Paused,
    Dead
}