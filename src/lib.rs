use bevy::prelude::*;
use wasm_bindgen::prelude::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Optional: bind Bevy to an existing HTML <canvas> element
                canvas: Some("#bevy-canvas".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        // Add your systems and plugins here
        .run();
}
