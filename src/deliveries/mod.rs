use bevy::app::App;
use bevy::prelude::{Component, Plugin, Update};
use landing_pad::collision;
use landing_pad::collision::DeliveryEventTrigger;

pub mod delivery;
pub mod landing_pad;
pub mod spawn;

#[derive(Component)]
pub struct DeliveriesPlugin;

impl Plugin for DeliveriesPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DeliveryEventTrigger>()
            .add_message::<spawn::SpawnPackageRequest>()
            .add_systems(
                Update,
                (
                    collision::observe_package_collisions,
                    spawn::despawn_package,
                    spawn::check_dying,
                    spawn::handle_spawn_requests,
                ),
            );
    }
}

mod config {

    pub const MASS: f32 = 50.0;
}
