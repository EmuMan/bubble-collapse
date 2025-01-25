use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Bubble {
    pub radius: f32,
    pub state: BubbleState,
}

#[derive(Component, Default, Debug)]
pub enum BubbleState {
    #[default]
    Moving,
    Popped,
}
