use bevy::prelude::*;

use bevy_egui::{egui, EguiContexts};

use crate::resources::bubbles;

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
