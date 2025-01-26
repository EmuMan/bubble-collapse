use bevy::prelude::*;

pub mod spawning;
pub mod combat;
pub mod shockwave;
pub mod movement;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum BubbleSystemSet {
    Spawning,
    Combat,
    Shockwave,
    Movement,
}
