use bevy::prelude::*;

#[derive(Event)]
pub struct MouseClickEvent {
    pub position: Vec2,
    pub window_position: Vec2,
}
