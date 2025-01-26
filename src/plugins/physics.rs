use bevy::prelude::*;

pub struct PhysicsPlugin;

use crate::{game_states::*, systems::physics::*};

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, update_with_velocity
            .run_if(in_state(GameState::InGame))
            .run_if(in_state(PausedState::Unpaused)));
    }
}
