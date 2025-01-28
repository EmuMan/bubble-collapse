use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand_core::RngCore;

use crate::components::bubbles::*;
use crate::components::physics::Collider;
use crate::resources::bubbles::*;
use crate::util::ActionTimer;

pub fn spawn_shockwaves(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut bubble_destroyed_event: EventReader<BubbleDestroyedEvent>,
) {
    for event in bubble_destroyed_event.read() {
        match event.bubble_type {
            BubbleType::Normal => {
                spawn_normal_shockwave(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    event.position,
                    event.radius,
                    event.color,
                );
            }
            BubbleType::Mega => {
                spawn_mega_shockwave(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    event.position,
                    event.radius,
                    event.color,
                );
            }
            BubbleType::ScatterShot => {
                spawn_scatter_shot(
                    &mut commands,
                    event.position,
                    200.0,
                    50.0,
                    event.color,
                );
            }
            BubbleType::Beam => {
                spawn_beam(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    event.position,
                    50.0,
                    event.color,
                )
            }
            BubbleType::BlackHole => {
                spawn_black_hole(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    event.position,
                    300.0,
                    event.color,
                );
            }
        }
    }
}

fn spawn_normal_shockwave(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    position: Vec2,
    radius: f32,
    color: Color,
) {
    let mut shockwave_color = color.clone();
    shockwave_color.set_alpha(0.3);
    commands.spawn(BubbleShockwaveBundle {
        mesh: Mesh2d(meshes.add(Circle::new(radius))),
        mesh_material: MeshMaterial2d(materials.add(shockwave_color)),
        transform: Transform::from_translation(position.extend(-(position.x / 1000.0 + position.y))),
        timed_effect: TimedEffect::new(Duration::from_secs_f32(0.2)),
        bubble_shockwave: BubbleShockwave::new(radius, 50.0),
        collider: Collider {
            radius,
            ..Default::default()
        },
    });
}

fn spawn_mega_shockwave(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    position: Vec2,
    radius: f32,
    color: Color,
) {
    let mut shockwave_color = color.clone();
    shockwave_color.set_alpha(0.5);
    commands.spawn(BubbleShockwaveBundle {
        mesh: Mesh2d(meshes.add(Circle::new(radius))),
        mesh_material: MeshMaterial2d(materials.add(shockwave_color)),
        transform: Transform::from_translation(position.extend(-(position.x / 1000.0 + position.y))),
        timed_effect: TimedEffect::new(Duration::from_secs_f32(1.0)),
        bubble_shockwave: BubbleShockwave::new(radius, 500.0),
        collider: Collider {
            radius,
            ..Default::default()
        },
    });
}

fn spawn_black_hole(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    position: Vec2,
    max_radius: f32,
    color: Color,
) {
    let mut black_hole_color = color.clone();
    black_hole_color.set_alpha(0.5);
    commands.spawn(BubbleBlackHoleBundle {
        mesh: Mesh2d(meshes.add(Circle::new(0.0))),
        mesh_material: MeshMaterial2d(materials.add(black_hole_color)),
        transform: Transform::from_translation(position.extend(-(position.x / 1000.0 + position.y))),
        timed_effect: TimedEffect::new(Duration::from_secs_f32(3.0)),
        bubble_black_hole: BubbleBlackHole::new(max_radius, 1000.0, 100.0),
        collider: Collider {
            radius: 0.0,
            ..Default::default()
        },
    });
}

fn spawn_scatter_shot(
    commands: &mut Commands,
    position: Vec2,
    radius: f32,
    variation: f32,
    color: Color,
) {
    let mut scatter_shot_color = color.clone();
    scatter_shot_color.set_alpha(0.5);

    commands.spawn(BubbleScatterShotSpawnerBundle {
        action_timer: ActionTimer::new(Duration::from_secs_f32(0.5), 15, TimerMode::Once),
        spawner: BubbleScatterShotSpawner::new(
            radius,
            variation,
            Duration::from_secs_f32(0.2),
            BubbleShockwave::new(0.0, 50.0),
            color,
        ),
        transform: Transform::from_translation(position.extend(0.0)),
    });
}

fn spawn_beam(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    position: Vec2,
    width: f32,
    color: Color,
) {
    let mut beam_color = color.clone();
    beam_color.set_alpha(0.5);

    commands.spawn(BubbleBeamBundle {
        mesh: Mesh2d(meshes.add(Rectangle::new(0.0, 1_000.0))),
        mesh_material: MeshMaterial2d(materials.add(beam_color)),
        timed_effect: TimedEffect::new(Duration::from_secs_f32(1.0)),
        beam: BubbleBeam::new(width),
        transform: Transform::from_translation(position.extend(-(position.x / 1000.0 + position.y))),
    });
}

pub fn expand_shockwaves(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut shockwave_query: Query<(
        Entity,
        &mut TimedEffect,
        &mut BubbleShockwave,
        &mut Collider,
        &Mesh2d,
        &MeshMaterial2d<ColorMaterial>,
    )>,
) {
    for (
        entity,
        mut timed_effect,
        mut shockwave,
        mut collider,
        mesh,
        material
    ) in &mut shockwave_query {
        if timed_effect.tick(time.delta()) {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        shockwave.set_radius_from_time(timed_effect.progress());
        collider.radius = shockwave.radius;
        meshes.insert(mesh, Circle::new(shockwave.radius).into());
        materials.get_mut(material).map(|mat| {
            mat.color.set_alpha((1.0 - timed_effect.progress()).powf(0.5) * 0.3);
        });
    }
}

pub fn wobble_black_holes(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut black_hole_query: Query<(
        Entity,
        &mut TimedEffect,
        &mut BubbleBlackHole,
        &mut Collider,
        &Mesh2d,
    )>,
) {
    for (
        entity,
        mut timed_effect,
        mut black_hole,
        mut collider,
        mesh
    ) in &mut black_hole_query {
        if timed_effect.tick(time.delta()) {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        black_hole.set_radius_from_time(timed_effect.progress());
        collider.radius = black_hole.radius;
        meshes.insert(mesh, Circle::new(black_hole.radius).into());
    }
}

pub fn spawn_scatter_shot_shockwaves(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut random: ResMut<GlobalEntropy<WyRand>>,
    mut scatter_shot_query: Query<(
        Entity,
        &mut ActionTimer,
        &mut BubbleScatterShotSpawner,
        &Transform,
    )>,
) {
    for (
        entity,
        mut action_timer,
        spawner,
        spawner_transform
    ) in &mut scatter_shot_query {
        let Some(to_spawn) = action_timer.tick(time.delta()) else {
            commands.entity(entity).despawn_recursive();
            continue;
        };

        for _ in 0..to_spawn {
            let angle = random.next_u32() as f32 / 1_000.0 % (PI * 2.0);
            let direction = Vec2::new(angle.cos(), angle.sin());
            let radius = spawner.radius + random.next_u32() as f32 / 1_000.0 % spawner.variation;
            let position = spawner_transform.translation + (direction * radius).extend(0.0);
            let mut color = spawner.shockwave_color.clone();
            color.set_alpha(0.99);

            commands.spawn(BubbleShockwaveBundle {
                mesh: Mesh2d(meshes.add(Circle::new(0.0))),
                mesh_material: MeshMaterial2d(materials.add(color)),
                transform: Transform::from_translation(position),
                timed_effect: spawner.instance_timer.clone(),
                bubble_shockwave: spawner.instance.clone(),
                collider: Collider {
                    radius: 0.0,
                    ..Default::default()
                },
            });
        }
    }
}

pub fn expand_beam(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut beam_query: Query<(
        Entity,
        &mut TimedEffect,
        &mut BubbleBeam,
        &Mesh2d
    )>,
) {
    for (
        entity,
        mut timed_effect,
        mut beam,
        mesh
    ) in &mut beam_query {
        if timed_effect.tick(time.delta()) {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        beam.set_width_from_time(timed_effect.progress());
        meshes.insert(mesh, Rectangle::new(beam.width, 3_000.0).into());
    }
}
