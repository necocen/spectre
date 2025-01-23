use bevy::{prelude::*, window::PrimaryWindow};
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

mod camera;
mod controller;
mod instancing;
pub mod tiles;
pub mod utils;

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Web版はブラウザ全体に表示
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            camera::CameraPlugin,
            instancing::CustomMaterialPlugin,
            controller::TilesControllerPlugin,
        ))
        .add_systems(Startup, set_window_title)
        .run();
}

fn set_window_title(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.title = "Infinite Spectres".to_string();
    }
}
