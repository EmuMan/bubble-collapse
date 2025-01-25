use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct GameStats {
    pub score: i32,
}
