use bevy::{prelude::*, window::PrimaryWindow};

use crate::resources::interaction::*;

pub fn handle_mouse_click(
    mut mouse_click_event: EventWriter<MouseClickEvent>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(window_pos) = window.cursor_position() {
            let world_pos = camera.viewport_to_world_2d(camera_transform, window_pos);
            if let Ok(world_pos) = world_pos {
                mouse_click_event.send(MouseClickEvent {
                    position: world_pos,
                    window_position: window_pos,
                });
            }
        }
    }
}
