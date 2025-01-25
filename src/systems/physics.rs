use bevy::prelude::*;

use crate::components::physics::*;

pub fn update_with_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += (velocity.velocity * time.delta().as_secs_f32()).extend(0.0);
    }
}
