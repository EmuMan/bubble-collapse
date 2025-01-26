use bevy::prelude::*;
use bevy_rand::prelude::GlobalEntropy;
use bevy_rand::prelude::WyRand;
use rand_core::RngCore;

use crate::components::bubbles::*;
use crate::components::physics::{Velocity, Collider};
use crate::resources::bubbles::BubbleSpawnTimer;

pub fn init_bubble_spawner(mut commands: Commands) {
    commands.insert_resource(BubbleSpawnTimer {
        timer: Timer::from_seconds(0.5, TimerMode::Repeating),
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
        let radius = 10.0;
        let x_pos = rng.next_u32() as f32 % 800.0 - 400.0;
        commands.spawn(BubbleBundle {
            mesh: Mesh2d(meshes.add(Circle::new(radius))),
            mesh_material: MeshMaterial2d(materials.add(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(x_pos, -100.0, 0.0)),
            bubble: Bubble::new(radius, 1.0),
            velocity: Velocity { velocity: Vec2::new(0.0, 100.0) },
            collider: Collider {
                radius,
                ..Default::default()
            }
        });
    }
}
