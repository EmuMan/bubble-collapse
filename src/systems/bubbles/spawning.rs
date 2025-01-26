use bevy::prelude::*;
use bevy_rand::prelude::GlobalEntropy;
use bevy_rand::prelude::WyRand;
use rand_core::RngCore;

use crate::components::bubbles::*;
use crate::resources::bubbles::BubbleSpawnTimer;

pub fn init_bubble_spawner(mut commands: Commands) {
    commands.insert_resource(BubbleSpawnTimer {
        timer: Timer::from_seconds(0.01, TimerMode::Repeating),
    });
}

pub fn spawn_bubbles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_timer: ResMut<BubbleSpawnTimer>,
    time: Res<Time>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    spawn_timer.timer.tick(time.delta());

    if spawn_timer.timer.just_finished() {
        let x_pos = rng.next_u32() as f32 / 500.0 % 800.0 - 400.0;
        let y_vel = rng.next_u32() as f32 % 100.0 + 50.0;
        let bubble_type = match rng.next_u32() % 300 {
            0 => BubbleType::Mega,
            1..=3 => BubbleType::ScatterShot,
            4..=6 => BubbleType::Beam,
            7..=8 => BubbleType::BlackHole,
            _ => BubbleType::Normal,
        };

        commands.spawn(BubbleBundle::from_type(
            &mut meshes,
            &mut materials,
            bubble_type,
            Vec2::new(x_pos, -400.0),
            Vec2::new(0.0, y_vel),
        ));
    }
}
