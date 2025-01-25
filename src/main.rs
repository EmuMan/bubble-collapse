use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use ggj2025::plugins::camera::CameraPlugin;
use ggj2025::plugins::stats::StatsPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 127.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(StatsPlugin)
        .run();
}
