use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_rand::prelude::EntropyPlugin;
use bevy_rand::prelude::WyRand;

use ggj2025::plugins::camera::CameraPlugin;
use ggj2025::plugins::{
    stats::StatsPlugin,
    bubbles::BubblesPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 127.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(CameraPlugin)
        .add_plugins(StatsPlugin)
        .add_plugins(BubblesPlugin)
        .run();
}
