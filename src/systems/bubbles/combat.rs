use bevy::prelude::*;

use crate::resources::bubbles::BubbleDestroyedEvent;
use crate::resources::interaction::*;
use crate::components::physics::*;
use crate::components::bubbles::*;

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
