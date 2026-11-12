use bevy::prelude::{Commands, KeyCode, Resource};

#[derive(Resource)]
pub struct ControlsResource {
    pub main_engine: KeyCode,
    pub left_engine: KeyCode,
    pub right_engine: KeyCode,
    pub reverse_engine: KeyCode,
    pub pause: KeyCode,
    pub tractor_beam: KeyCode,
    pub reset: KeyCode,
}

impl Default for ControlsResource {
    fn default() -> Self {
        Self {
            main_engine: KeyCode::ArrowUp,
            left_engine: KeyCode::ArrowRight,
            right_engine: KeyCode::ArrowLeft,
            reverse_engine: KeyCode::ArrowDown,
            pause: KeyCode::Escape,
            tractor_beam: KeyCode::Tab,
            reset: KeyCode::F1
        }
    }
}

pub fn initialize_controls(mut command: Commands) {
    command.insert_resource(ControlsResource::default());
}
