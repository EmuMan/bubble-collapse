use bevy::prelude::*;

use bevy_egui::{egui, EguiContexts};

use crate::{game_states::DebugState, resources::bubbles};

pub fn ui_debug(
    mut contexts: EguiContexts,
    // mut stats: ResMut<stats::GameStats>,
    mut bubble_spawn_timer: ResMut<bubbles::BubbleSpawnTimer>,
    // mut bubble_chances: ResMut<bubbles::BubbleChances>,
) {
    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        let spawn_rate_label = ui.label("Spawn Rate");
        ui.add(egui::DragValue::new(&mut bubble_spawn_timer.action_timer.amount)).labelled_by(spawn_rate_label.id);
    });
}

pub fn toggle_debug_on_on_backslash(
    mut debug_state: ResMut<NextState<DebugState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Backslash) {
        debug_state.set(DebugState::Debug);
    }
}

pub fn toggle_debug_off_on_backslash(
    mut debug_state: ResMut<NextState<DebugState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Backslash) {
        debug_state.set(DebugState::NoDebug);
    }
}
