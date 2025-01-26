use bevy::prelude::*;

use crate::systems::ui::*;
use crate::game_states::{DebugState, GameState, PausedState};

pub struct UiPlugin;

impl Plugin for UiPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                (
                    debug::ui_debug,
                    debug::toggle_debug_off_on_backslash,
                )
                    .run_if(in_state(DebugState::Debug)),
                debug::toggle_debug_on_on_backslash
                    .run_if(in_state(DebugState::NoDebug)),
            ))
            .add_systems(OnEnter(GameState::MainMenu), main_menu::draw_main_menu)
            .add_systems(OnExit(GameState::MainMenu), main_menu::cleanup_main_menu)
            .add_systems(Update, (
                main_menu::button_system,
            ).run_if(in_state(GameState::MainMenu)))
            .add_systems(OnEnter(PausedState::Paused), pause_menu::draw_pause_menu)
            .add_systems(OnExit(PausedState::Paused), pause_menu::cleanup_pause_menu)
            .add_systems(Update, (
                pause_menu::button_system,
            ).run_if(in_state(PausedState::Paused)))
            .add_systems(Update, (
                pause_menu::pause_game_on_esc.run_if(in_state(PausedState::Unpaused)),
                pause_menu::unpause_game_on_esc.run_if(in_state(PausedState::Paused)),
            ).run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), pause_menu::unpause_game)
            .add_systems(OnEnter(GameState::InGame), upgrades_menu::draw_upgrades_menu)
            .add_systems(OnExit(GameState::InGame), upgrades_menu::cleanup_upgrades_menu)
            .add_systems(Update, (
                upgrades_menu::button_system,
            ).run_if(in_state(GameState::InGame)));
    }

}
