use std::time::Duration;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rand::prelude::GlobalEntropy;
use bevy_rand::prelude::WyRand;
use rand_core::RngCore;

use crate::components::bubbles::*;
use crate::resources::bubbles::*;
use crate::util::get_viewport_bounds;
use crate::util::ActionTimer;

pub fn init_bubble_spawner(
    mut commands: Commands,
    mut bubble_chances: ResMut<BubbleChances>,
) {
    commands.insert_resource(BubbleSpawnTimer {
        action_timer: ActionTimer::new(
            Duration::from_secs_f32(1.0),
            10,
            TimerMode::Repeating,
        ),
    });

    bubble_chances.set_chance(BubbleType::Normal, 100.0);
    bubble_chances.set_chance(BubbleType::Mega, 0.0);
    bubble_chances.set_chance(BubbleType::ScatterShot, 0.0);
    bubble_chances.set_chance(BubbleType::BlackHole, 0.0);
    bubble_chances.set_chance(BubbleType::Beam, 0.0);
}

pub fn spawn_bubbles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_timer: ResMut<BubbleSpawnTimer>,
    time: Res<Time>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    chances: Res<BubbleChances>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let screen_bounds = get_viewport_bounds(&window_query, &camera_query)
        .unwrap_or(Rect {
            min: Vec2::new(-400.0, -400.0),
            max: Vec2::new(500.0, 500.0),
        });

    let to_spawn = spawn_timer.action_timer.tick(time.delta()).unwrap_or(0);

    for _ in 0..to_spawn {
        let x_pos = rng.next_u32() as f32 / 500.0 % (screen_bounds.max.x - screen_bounds.min.x)
            + screen_bounds.min.x;
        let y_vel = rng.next_u32() as f32 % 100.0 + 50.0;
        let bubble_type = chances.random_sample(rng.next_u32());

        commands.spawn(BubbleBundle::from_type(
            &mut meshes,
            &mut materials,
            bubble_type,
            Vec2::new(x_pos, screen_bounds.min.y - 50.0),
            Vec2::new(0.0, y_vel),
        ));
    }
}

pub fn despawn_bubbles(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    bubble_query: Query<(Entity, &Transform), With<Bubble>>,
) {
    let boundary = get_viewport_bounds(&window_query, &camera_query)
        .map(|bounds| bounds.max.y)
        .unwrap_or(0.0);
    for (entity, transform) in bubble_query.iter() {
        if transform.translation.y > boundary + 50.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn cleanup_everything(
    mut commands: Commands,
    query: Query<Entity, Or<(
        With<Bubble>,
        With<BubbleShockwave>,
        With<BubbleBlackHole>,
        With<BubbleBeam>,
        With<BubbleScatterShotSpawner>,
    )>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
