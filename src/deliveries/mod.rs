use crate::delivery_target::collision;
use crate::delivery_target::collision::PackageDeliveredTrigger;
use bevy::app::App;
use bevy::prelude::{Component, Plugin, Update};
use crate::{deliveries, delivery_target};

pub mod delivery;
pub mod spawn;

#[derive(Component)]
pub struct DeliveriesPlugin;

impl Plugin for DeliveriesPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PackageDeliveredTrigger>()
            .add_message::<spawn::SpawnPackageRequest>()
            .add_systems(Update, (
                collision::observe_package_collisions,
            deliveries::spawn::despawn_package,
                spawn::handle_spawn_requests
            ));
    }
}

mod config {

    pub const MASS: f32 = 50.0;
}
