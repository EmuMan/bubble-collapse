use std::collections::HashMap;

use bevy::prelude::*;

use crate::{components::bubbles::*, util::ActionTimer};

#[derive(Resource, Debug, Default)]
pub struct BubbleSpawnTimer {
    pub action_timer: ActionTimer,
}

#[derive(Resource, Debug, Default)]
pub struct BubbleChances {
    pub chances: HashMap<BubbleType, f32>,
}

impl BubbleChances {
    pub fn new() -> Self {
        Self {
            chances: HashMap::new(),
        }
    }

    pub fn get_chance(&self, bubble_type: BubbleType) -> f32 {
        *self.chances.get(&bubble_type).unwrap_or(&0.0)
    }

    pub fn set_chance(&mut self, bubble_type: BubbleType, chance: f32) {
        self.chances.insert(bubble_type, chance);
    }

    pub fn add_chance(&mut self, bubble_type: BubbleType, chance: f32) {
        let current = self.get_chance(bubble_type);
        self.set_chance(bubble_type, current + chance);
    }

    pub fn random_sample(&self, random: u32) -> BubbleType {
        let total = self.chances.values().sum::<f32>();
        let mut chance = random as f32 / u32::MAX as f32 * total;
        for (bubble_type, bubble_chance) in self.chances.iter() {
            chance -= bubble_chance;
            if chance <= 0.0 {
                return *bubble_type;
            }
        }
        BubbleType::Normal
    }
}

#[derive(Event, Debug, Default)]
pub struct BubbleDestroyedEvent {
    pub position: Vec2,
    pub radius: f32,
    pub color: Color,
    pub bubble_type: BubbleType,
}

#[derive(Event, Debug, Default)]
pub struct BubbleCollapsedEvent {
    pub triggered_by_user: bool,
    pub score_change: i32,
}
