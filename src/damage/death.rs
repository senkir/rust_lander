use crate::damage::explosion::DeathTrigger;
use crate::damage::health::Health;
use crate::lander::lander::Lander;
use crate::state::GameState;
use bevy::log::info;
use bevy::prelude::{Commands, Component, Entity, Has, MessageWriter, NextState, Query, Res, ResMut, Resource, With};
use bevy::time::{Time, Timer, TimerMode};
use std::time::Duration;
use crate::camera::trauma::CameraShake;
use crate::deliveries::delivery::Delivery;
use crate::deliveries::landing_pad::collision::DeliveryEventTrigger;
use crate::hud::score::display::ScoreUpdate;

#[derive(Component)]
pub struct Dying {
    timer: Timer,
    entity: Entity,
    kind: DeathKind
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum DeathKind {
    Lander,
    Package
}

pub fn observe_health(
    mut commands: Commands,
    mut query: Query<(&mut Health, Entity, Has<Lander>)>,
    mut writer: MessageWriter<DeathTrigger>,
) {
    for (mut health, entity, is_lander) in &mut query.iter_mut() {
        if (health.is_dead_not_dying()) {
            //mark for death
            health.mark_for_death();

            let dying = Dying {
                timer: Timer::new(Duration::from_millis(1500),
                                  TimerMode::Once),
                entity,
                kind: if (is_lander) {
                    DeathKind::Lander
                } else {
                    DeathKind::Package
                }
            };
            info!("marking for death? Entity: {:?}, is_lander? {}", entity, is_lander);
            writer.write(DeathTrigger {
                entity,
                kind: dying.kind,
            });
            commands.entity(entity).insert(dying);
        }
    }
}

pub fn countdown_death_timer(
    time: Res<Time>,
    mut shake: ResMut<CameraShake>,
    mut state: ResMut<NextState<GameState>>,
    mut writer: MessageWriter<DeliveryEventTrigger>,
    mut query: Query<(&mut Dying)>,
) {
    //TODO: handle if just the package is dying
    for (mut dying) in &mut query.iter_mut() {
        if !dying.timer.tick(time.delta()).is_finished() {
            //not ready
            continue;
        }

        shake.trauma = 0.0;
        if (dying.kind == DeathKind::Lander) {
            state.set(GameState::Dead);
        } else {
            //trigger despawn
            writer.write(DeliveryEventTrigger {
                entity: dying.entity,
                success: false
            });
        }
    }
}
