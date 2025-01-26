use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct BubbleSpawnTimer {
    pub timer: Timer,
}

#[derive(Event, Debug, Default)]
pub struct BubbleDestroyedEvent {
    pub position: Vec2,
    pub radius: f32,
    pub color: Color,
}
