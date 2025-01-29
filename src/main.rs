#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::winit::WinitWindows;
use bevy_egui::EguiPlugin;
use bevy_rand::prelude::EntropyPlugin;
use bevy_rand::prelude::WyRand;

use bevy_framepace::FramepacePlugin;

use bubble_collapse::plugins::camera::CameraPlugin;
use bubble_collapse::plugins::{
    game_states::GameStatesPlugin,
    stats::StatsPlugin,
    bubbles::BubblesPlugin,
    physics::PhysicsPlugin,
    interaction::InteractionPlugin,
    ui::UiPlugin,
    cache::CachePlugin,
};
use winit::window::Icon;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.3, 0.5, 0.8)))
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: bevy::asset::AssetMetaCheck::Never,
            ..default()
        }).set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bubble Collapse".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, set_window_icon)
        .add_plugins(EguiPlugin)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(FramepacePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(CachePlugin)
        .add_plugins(GameStatesPlugin)
        .add_plugins(StatsPlugin)
        .add_plugins(BubblesPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(InteractionPlugin)
        .run();
}

// from https://bevy-cheatbook.github.io/window/icon.html
fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icon.png");
        let Ok(image) = image else {
            bevy::log::warn!("Failed to open icon path");
            return;
        };
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}
