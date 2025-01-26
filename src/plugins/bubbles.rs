use bevy::prelude::*;

use crate::resources::bubbles::*;
use crate::systems::bubbles::spawning::*;
use crate::systems::bubbles::combat::*;
use crate::systems::bubbles::shockwave::*;
use crate::systems::bubbles::movement::*;
use crate::systems::bubbles::BubbleSystemSet;
use crate::game_states::{GameState, PausedState};

pub struct BubblesPlugin;

impl Plugin for BubblesPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<BubbleSpawnTimer>()
            .init_resource::<BubbleChances>()
            .add_event::<BubbleDestroyedEvent>()
            .add_systems(OnEnter(GameState::InGame), init_bubble_spawner)
            .add_systems(OnExit(GameState::InGame), cleanup_everything)
            .add_systems(Update, (
                (
                    spawn_bubbles,
                ).in_set(BubbleSystemSet::Spawning)
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(PausedState::Unpaused)),
                (
                    update_bubble_velocity,
                ).in_set(BubbleSystemSet::Movement)
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(PausedState::Unpaused)),
                (
                    bubble_clicked,
                    bubble_hit_by_shockwave,
                    bubble_in_black_hole,
                    bubble_hit_by_beam,
                    advance_bubble_collapse
                        .after(bubble_clicked)
                        .after(bubble_hit_by_shockwave)
                        .after(bubble_in_black_hole)
                        .after(bubble_hit_by_beam),
                ).in_set(BubbleSystemSet::Combat)
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(PausedState::Unpaused)),
                (
                    spawn_shockwaves,
                    expand_shockwaves,
                    wobble_black_holes,
                    spawn_scatter_shot_shockwaves,
                    expand_beam,
                ).in_set(BubbleSystemSet::Shockwave)
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(PausedState::Unpaused)),
            ));
    }

}
