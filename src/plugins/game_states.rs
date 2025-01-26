use bevy::prelude::*;

use crate::game_states::*;

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .init_state::<DebugState>()
            .init_state::<PausedState>();
    }

}
