use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct BubbleSpawnTimer {
    pub timer: Timer,
}
