use bevy::prelude::*;

use crate::components::bubbles::*;
use crate::components::Speed;

pub fn move_bubbles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Speed), With<Bubble>>,
) {
    for (mut transform, speed) in query.iter_mut() {
        transform.translation.y += speed.speed * time.delta().as_secs_f32();
    }
}
