use bevy::prelude::*;

use crate::game_states::GameState;
use crate::resources::stats::*;
use crate::systems::stats::*;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameStats>()
            .add_systems(OnEnter(GameState::InGame), (
                init_stats,
                draw_score,
            ))
            .add_systems(OnExit(GameState::InGame), cleanup_score)
            .add_systems(Update, (
                update_score,
                increment_score_for_destroyed_bubbles,
            ).run_if(in_state(GameState::InGame)));
    }

}
