use bevy::prelude::*;

use crate::systems::ui::*;
use crate::game_states::{DebugState, GameState};

pub struct UiPlugin;

impl Plugin for UiPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, debug::ui_debug
                .run_if(in_state(DebugState::Debug)))
            .add_systems(OnEnter(GameState::MainMenu), main_menu::draw_main_menu)
            .add_systems(OnExit(GameState::MainMenu), main_menu::cleanup_main_menu)
            .add_systems(Update, (
                main_menu::button_system,
            ).run_if(in_state(GameState::MainMenu)));
    }

}
