use bevy::app::App;
use bevy::prelude::Update;
use bevy::prelude::{
    Component, Deref, DerefMut, Plugin, Query, Res, Sprite, Time, Timer, TimerMode,
};

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AnimationMode {
    Repeating,
    Once,
}

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
    repeating: AnimationMode,
}

impl AnimationIndices {
    fn has_finished(&self, index: usize) -> bool {
        self.last == index
    }

    /// generate the next frame index
    pub fn next(&self, index: usize) -> usize {
        if self.has_finished(index) {
            self.first
        } else if (index < self.first) {
            self.first
        } else {
            index + 1
        }
    }

    /// determine if the animation has finished playing
    /// NOTE: if AnimationMode::Repeating this will never be true
    pub fn animation_finished(&self, index: usize) -> bool {
        self.last == index && self.repeating == AnimationMode::Once
    }
}

impl AnimationIndices {
    pub fn new(first: usize, last: usize, repeating: AnimationMode) -> Self {
        Self {
            first,
            last,
            repeating,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer {
    timer: Timer,
}

impl AnimationTimer {
    //determine animation speed
    pub fn from_seconds(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
        }
    }
}

/// run the animation for anything matching the given properties
pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if (indices.animation_finished(atlas.index)) {
                return;
            }
            atlas.index = indices.next(atlas.index);
        }
    }
}

#[derive(Component)]
pub struct AutoAnimatorPlugin;

impl Plugin for AutoAnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite);
    }
}
