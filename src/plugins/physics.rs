use bevy::prelude::*;

pub struct PhysicsPlugin;

use crate::systems::physics::*;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, update_with_velocity);
    }
}
