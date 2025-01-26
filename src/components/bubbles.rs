use std::time::Duration;

use bevy::prelude::*;

use crate::components::physics::{Collider, Velocity};

#[derive(Component, Default, Debug)]
pub struct Bubble {
    pub radius: f32,
    pub initial_radius: f32,
    pub state: BubbleState,
    pub bubble_type: BubbleType,
    pub collapse_timer: Timer,
}

impl Bubble {
    pub fn new(radius: f32, collapse_time: f32, bubble_type: BubbleType) -> Self {
        Self {
            radius,
            initial_radius: radius,
            state: BubbleState::Moving,
            bubble_type,
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

#[derive(Default, Debug, PartialEq)]
pub enum BubbleState {
    #[default]
    Moving,
    Popped,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum BubbleType {
    #[default]
    Normal,
    Mega,
    ScatterShot,
    Beam,
    BlackHole,
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

impl BubbleBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        radius: f32,
        color: Color,
        bubble_type: BubbleType,
        collapse_time: f32,
        pos: Vec2,
        velocity: Vec2,
    ) -> BubbleBundle {
        BubbleBundle {
            mesh: Mesh2d(meshes.add(Circle::new(radius))),
            mesh_material: MeshMaterial2d(materials.add(color)),
            transform: Transform::from_translation(pos.extend(0.0)),
            bubble: Bubble::new(radius, collapse_time, bubble_type),
            velocity: Velocity { velocity },
            collider: Collider {
                radius,
                ..Default::default()
            },
        }
    }

    pub fn from_type(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        bubble_type: BubbleType,
        pos: Vec2,
        velocity: Vec2,
    ) -> BubbleBundle {
        let (radius, color, collapse_time) = match bubble_type {
            BubbleType::Normal => (10.0, Color::WHITE, 0.0),
            BubbleType::Mega => (30.0, Color::linear_rgb(1.0, 0.0, 0.0), 2.0),
            BubbleType::ScatterShot => (10.0, Color::linear_rgb(0.0, 1.0, 0.0), 0.5),
            BubbleType::Beam => (10.0, Color::linear_rgb(0.0, 0.0, 1.0), 1.0),
            BubbleType::BlackHole => (10.0, Color::BLACK, 5.0),
        };
        BubbleBundle::new(
            meshes,
            materials,
            radius,
            color,
            bubble_type,
            collapse_time,
            pos,
            velocity,
        )
    }
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
