use bevy::prelude::*;

use crate::components::physics::{Collider, Velocity};

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

    pub fn tick(&mut self, dt: f32) -> bool {
        self.radius += self.speed * dt;
        if self.decay {
            let time = (self.radius - self.initial_radius) / (self.max_radius - self.initial_radius);
            self.strength = self.initial_strength * (1.0 - time);
        }
        self.radius > self.max_radius
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
