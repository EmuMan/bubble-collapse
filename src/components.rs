use bevy::prelude::*;

pub mod bubbles;

#[derive(Component, Debug, Default)]
pub struct Speed {
    pub speed: f32,
}
