use bevy::prelude::Component;

#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
pub enum TractorState {
    #[default]
    Off,
    On
}

#[derive(Component, PartialEq, Eq)]
pub struct ShipTractor {
    pub state: TractorState,
}

impl Default for ShipTractor {
    fn default() -> Self {
        Self {
            state: TractorState::Off,
        }
    }
}
