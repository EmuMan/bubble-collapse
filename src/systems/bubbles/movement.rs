use bevy::prelude::*;

use crate::components::{bubbles::Bubble, physics::Velocity};

pub fn update_bubble_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Bubble)>,
) {
    for (mut velocity, bubble) in query.iter_mut() {
        velocity.velocity.x = if velocity.velocity.x < 0.0 {
            let new_val = velocity.velocity.x + time.delta().as_secs_f32() * 10.0;
            if new_val > 0.0 {
                0.0
            } else {
                new_val
            }
        } else {
            let new_val = velocity.velocity.x - time.delta().as_secs_f32() * 10.0;
            if new_val < 0.0 {
                0.0
            } else {
                new_val
            }
        };

        velocity.velocity.y += time.delta().as_secs_f32() * 10.0;
        if velocity.velocity.y > bubble.max_y_velocity {
            velocity.velocity.y = bubble.max_y_velocity;
        }
    }
}
