use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_rand::prelude::EntropyPlugin;
use bevy_rand::prelude::WyRand;

use bevy_framepace::FramepacePlugin;

use ggj2025::plugins::camera::CameraPlugin;
use ggj2025::plugins::{
    stats::StatsPlugin,
    bubbles::BubblesPlugin,
    physics::PhysicsPlugin,
    interaction::InteractionPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 127.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(FramepacePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(StatsPlugin)
        .add_plugins(BubblesPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(InteractionPlugin)
        .run();
}
