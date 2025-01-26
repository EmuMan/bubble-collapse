use std::time::Duration;

use bevy::prelude::*;

use crate::components::physics::{Collider, Velocity};

#[derive(Component, Default, Debug)]
pub struct Bubble {
    pub radius: f32,
    pub initial_radius: f32,
    pub state: BubbleState,
    pub collapse_timer: Timer,
}

impl Bubble {
    pub fn new(radius: f32, collapse_time: f32) -> Self {
        Self {
            radius,
            initial_radius: radius,
            state: BubbleState::Moving,
            collapse_timer: Timer::from_seconds(collapse_time, TimerMode::Once),
        }
    }

    pub fn collapse(&mut self) {
        self.state = BubbleState::Popped;
        self.collapse_timer.reset();
        self.initial_radius = self.radius;
    }

    pub fn update_collapse(&mut self, time: &Duration) -> Option<f32> {
        self.collapse_timer.tick(*time);
        if self.collapse_timer.finished() {
            return None;
        }
        Some(self.collapse_timer.remaining_secs() / self.collapse_timer.duration().as_secs_f32())
    }
}

#[derive(Component, Default, Debug, PartialEq)]
pub enum BubbleState {
    #[default]
    Moving,
    Popped,
}

#[derive(Bundle)]
pub struct BubbleBundle {
    pub mesh: Mesh2d,
    pub mesh_material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub bubble: Bubble,
    pub velocity: Velocity,
    pub collider: Collider,
}


#[derive(Component, Default, Debug)]
pub struct BubbleShockwave {
    initial_radius: f32,
    pub radius: f32,
    pub speed: f32,
    pub max_radius: f32,
    initial_strength: f32,
    pub strength: f32,
    pub decay: bool,
}

impl BubbleShockwave {
    pub fn new(radius: f32, speed: f32, max_radius: f32, initial_strength: f32, decay: bool) -> Self {
        Self {
            initial_radius: radius,
            radius,
            speed,
            max_radius,
            initial_strength,
            strength: initial_strength,
            decay,
        }
    }

    pub fn tick(&mut self, dt: f32) -> Option<f32> {
        self.radius += self.speed * dt;
        let time = (self.radius - self.initial_radius) / (self.max_radius - self.initial_radius);
        if self.decay {
            self.strength = self.initial_strength * (1.0 - time);
        }
        if self.radius < self.max_radius {
            Some(time)
        } else {
            None
        }
    }
}

#[derive(Bundle)]
pub struct BubbleShockwaveBundle {
    pub mesh: Mesh2d,
    pub mesh_material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub bubble_shockwave: BubbleShockwave,
    pub collider: Collider,
}
