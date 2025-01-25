use bevy::prelude::*;

use crate::resources::bubbles::BubbleDestroyedEvent;
use crate::resources::interaction::*;
use crate::components::physics::*;
use crate::components::bubbles::*;
use crate::util;

pub fn bubble_clicked(
    mut commands: Commands,
    mut mouse_click_events: EventReader<MouseClickEvent>,
    mut bubble_destroyed_event: EventWriter<BubbleDestroyedEvent>,
    mut bubble_query: Query<(Entity, &Transform, &Collider), With<Bubble>>,
) {
    for event in mouse_click_events.read() {
        for (entity, transform, collider) in bubble_query.iter_mut() {
            if collider.is_point_inside(transform.translation.truncate(), event.position) {
                commands.entity(entity).despawn();
                bubble_destroyed_event.send(BubbleDestroyedEvent {
                    position: transform.translation.truncate(),
                    radius: collider.radius,
                });
            }
        }
    }
}

pub fn bubble_hit_by_shockwave(
    mut commands: Commands,
    time: Res<Time>,
    mut bubble_destroyed_event: EventWriter<BubbleDestroyedEvent>,
    mut shockwave_query: Query<(&Transform, &Collider), With<BubbleShockwave>>,
    mut bubble_query: Query<(Entity, &Transform, &Collider, &Velocity), With<Bubble>>,
) {
    for (shockwave_transform, shockwave_collider) in shockwave_query.iter_mut() {
        for (bubble_entity, bubble_transform, bubble_collider, bubble_velocity) in bubble_query.iter_mut() {
            if util::continuous_circle_collision(
                shockwave_transform.translation.truncate(),
                Vec2::ZERO,
                shockwave_collider.radius,
                bubble_transform.translation.truncate(),
                bubble_velocity.velocity,
                bubble_collider.radius,
                time.delta().as_secs_f32(),
            ) {
                commands.entity(bubble_entity).despawn();
                bubble_destroyed_event.send(BubbleDestroyedEvent {
                    position: bubble_transform.translation.truncate(),
                    radius: bubble_collider.radius,
                });
            }
        }
    }
}
