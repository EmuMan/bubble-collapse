use bevy::prelude::*;

use crate::resources::bubbles::BubbleDestroyedEvent;
use crate::resources::interaction::*;
use crate::components::physics::*;
use crate::components::bubbles::*;
use crate::util;

pub fn advance_bubble_collapse(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Assets<ColorMaterial>>,
    mut bubble_destroyed_event: EventWriter<BubbleDestroyedEvent>,
    mut bubble_query: Query<(Entity, &mut Bubble, &Transform, &mut Collider, &Mesh2d, &MeshMaterial2d<ColorMaterial>)>,
) {
    for (entity, mut bubble, transform, mut collider, mesh, material) in bubble_query.iter_mut() {
        if bubble.state == BubbleState::Popped {
            match bubble.update_collapse(&time.delta()) {
                Some(progress) => {
                    let new_radius = bubble.initial_radius * progress.powf(0.33);
                    bubble.radius = new_radius;
                    collider.radius = new_radius;
                    meshes.insert(mesh, Circle::new(new_radius).into());
                }
                None => {
                    commands.entity(entity).despawn();
                    bubble_destroyed_event.send(BubbleDestroyedEvent {
                        position: transform.translation.truncate(),
                        radius: collider.radius,
                        color: materials.get(material).map(|mat| mat.color).unwrap_or(Color::WHITE),
                        bubble_type: bubble.bubble_type,
                    });
                }
            }
        }
    }
}

pub fn bubble_clicked(
    mut mouse_click_events: EventReader<MouseClickEvent>,
    mut bubble_query: Query<(&Transform, &Collider, &mut Bubble)>,
) {
    for event in mouse_click_events.read() {
        for (transform, collider, mut bubble) in bubble_query.iter_mut() {
            if bubble.state == BubbleState::Popped {
                continue;
            }
            if collider.is_point_inside(transform.translation.truncate(), event.position) {
                bubble.collapse();
            }
        }
    }
}

pub fn bubble_hit_by_shockwave(
    time: Res<Time>,
    mut shockwave_query: Query<(&Transform, &Collider), With<BubbleShockwave>>,
    mut bubble_query: Query<(&Transform, &Collider, &Velocity, &mut Bubble)>,
) {
    for (shockwave_transform, shockwave_collider) in shockwave_query.iter_mut() {
        for (
            bubble_transform,
            bubble_collider,
            bubble_velocity,
            mut bubble
        ) in bubble_query.iter_mut() {
            if bubble.state == BubbleState::Popped {
                continue;
            }
            if util::continuous_circle_collision(
                shockwave_transform.translation.truncate(),
                Vec2::ZERO,
                shockwave_collider.radius,
                bubble_transform.translation.truncate(),
                bubble_velocity.velocity,
                bubble_collider.radius,
                time.delta().as_secs_f32(),
            ) {
                bubble.collapse();
            }
        }
    }
}

pub fn bubble_in_black_hole(
    time: Res<Time>,
    mut black_hole_query: Query<(&Transform, &Collider, &BubbleBlackHole)>,
    mut bubble_query: Query<(&Transform, &Collider, &mut Velocity, &mut Bubble)>,
) {
    for (black_hole_transform, black_hole_collider, black_hole) in black_hole_query.iter_mut() {
        for (
            bubble_transform,
            bubble_collider,
            mut bubble_velocity,
            mut bubble
        ) in bubble_query.iter_mut() {
            if util::continuous_circle_collision(
                black_hole_transform.translation.truncate(),
                Vec2::ZERO,
                black_hole_collider.radius,
                bubble_transform.translation.truncate(),
                bubble_velocity.velocity,
                bubble_collider.radius,
                time.delta().as_secs_f32(),
            ) {
                let direction = black_hole_transform.translation.truncate() - bubble_transform.translation.truncate();
                let distance = direction.length();
                let force = black_hole.strength / distance;
                bubble_velocity.velocity += direction.normalize() * force;
                if distance < black_hole.max_radius / 10.0 {
                    bubble.collapse();
                }
            }
        }
    }
}
