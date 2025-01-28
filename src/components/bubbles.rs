use std::time::Duration;

use bevy::prelude::*;

use crate::{components::physics::{Collider, Velocity}, util::ActionTimer};

#[derive(Component, Default, Debug)]
pub struct Bubble {
    pub radius: f32,
    pub initial_radius: f32,
    pub state: BubbleState,
    pub bubble_type: BubbleType,
    pub collapse_timer: Timer,
    pub max_y_velocity: f32,
}

impl Bubble {
    pub fn new(radius: f32, collapse_time: f32, bubble_type: BubbleType, max_y_velocity: f32) -> Self {
        Self {
            radius,
            initial_radius: radius,
            state: BubbleState::Moving,
            bubble_type,
            collapse_timer: Timer::from_seconds(collapse_time, TimerMode::Once),
            max_y_velocity,
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
        Some(self.collapse_timer.elapsed_secs() / self.collapse_timer.duration().as_secs_f32())
    }
}

#[derive(Default, Debug, PartialEq)]
pub enum BubbleState {
    #[default]
    Moving,
    Popped,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            transform: Transform::from_translation(pos.extend(-(pos.x / 1000.0 + pos.y))),
            bubble: Bubble::new(radius, collapse_time, bubble_type, velocity.y),
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
            BubbleType::Mega => (30.0, Color::linear_rgb(1.0, 0.0, 0.0), 1.0),
            BubbleType::ScatterShot => (20.0, Color::linear_rgb(0.0, 1.0, 0.0), 0.5),
            BubbleType::Beam => (15.0, Color::linear_rgb(1.0, 1.0, 0.0), 0.5),
            BubbleType::BlackHole => (10.0, Color::BLACK, 2.0),
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

#[derive(Component, Default, Debug, Clone)]
pub struct TimedEffect {
    pub timer: Timer,
}

impl TimedEffect {
    pub fn new(duration: Duration) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
        }
    }

    pub fn tick(&mut self, time: Duration) -> bool {
        self.timer.tick(time).finished()
    }

    pub fn reset(&mut self) {
        self.timer.reset();
    }

    pub fn set_duration(&mut self, duration: &Duration) {
        self.timer.set_duration(*duration);
    }

    pub fn duration(&self) -> Duration {
        self.timer.duration()
    }

    pub fn elapsed(&self) -> Duration {
        self.timer.elapsed()
    }

    pub fn remaining(&self) -> Duration {
        self.timer.remaining()
    }

    pub fn progress(&self) -> f32 {
        self.elapsed().as_secs_f32() / self.duration().as_secs_f32()
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct BubbleShockwave {
    initial_radius: f32,
    pub radius: f32,
    pub max_radius: f32,
}

impl BubbleShockwave {
    pub fn new(radius: f32, max_radius: f32) -> Self {
        Self {
            initial_radius: radius,
            radius,
            max_radius,
        }
    }

    pub fn set_radius_from_time(&mut self, time: f32) {
        self.radius = self.initial_radius.lerp(self.max_radius, time);
    }
}

#[derive(Bundle, Clone)]
pub struct BubbleShockwaveBundle {
    pub mesh: Mesh2d,
    pub mesh_material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub timed_effect: TimedEffect,
    pub bubble_shockwave: BubbleShockwave,
    pub collider: Collider,
}

#[derive(Component, Default, Debug)]
pub struct BubbleBlackHole {
    pub max_radius: f32,
    pub radius: f32,
    pub strength: f32,
    pub max_pull: f32,
}

impl BubbleBlackHole {
    pub fn new(max_radius: f32, strength: f32, max_pull: f32) -> Self {
        Self {
            max_radius,
            radius: 0.0,
            strength,
            max_pull,
        }
    }

    pub fn set_radius_from_time(&mut self, time: f32) {
        let up_n_down_bit = -1.0 * (time * 2.34 - 1.17).powi(10) + 5.0;
        let wobbly_bit = (-1.0 * (time * 56.4).cos() + 1.0) / 2.0;
        self.radius = self.max_radius * (up_n_down_bit + wobbly_bit) / 6.0;
    }
}

#[derive(Bundle)]
pub struct BubbleBlackHoleBundle {
    pub mesh: Mesh2d,
    pub mesh_material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub timed_effect: TimedEffect,
    pub bubble_black_hole: BubbleBlackHole,
    pub collider: Collider,
}

#[derive(Component)]
pub struct BubbleScatterShotSpawner {
    pub radius: f32,
    pub variation: f32,
    pub instance_timer: TimedEffect,
    pub instance: BubbleShockwave,
    pub shockwave_color: Color,
}

impl BubbleScatterShotSpawner {
    pub fn new(
        radius: f32,
        variation: f32,
        instance_duration: Duration,
        instance: BubbleShockwave,
        shockwave_color: Color,
    ) -> Self {
        Self {
            radius,
            variation,
            instance_timer: TimedEffect::new(instance_duration),
            instance,
            shockwave_color,
        }
    }
}

#[derive(Bundle)]
pub struct BubbleScatterShotSpawnerBundle {
    pub action_timer: ActionTimer,
    pub spawner: BubbleScatterShotSpawner,
    pub transform: Transform,
}

#[derive(Component, Debug, Default)]
pub struct BubbleBeam {
    pub width: f32,
    pub max_width: f32,
}

impl BubbleBeam {
    pub fn new(max_width: f32) -> Self {
        Self {
            width: 0.0,
            max_width,
        }
    }

    pub fn set_width_from_time(&mut self, time: f32) {
        let value = (4.0 * time).powi(2).min(-time + 1.0);
        self.width = self.max_width * value;
    }
}

#[derive(Bundle)]
pub struct BubbleBeamBundle {
    pub mesh: Mesh2d,
    pub mesh_material: MeshMaterial2d<ColorMaterial>,
    pub timed_effect: TimedEffect,
    pub beam: BubbleBeam,
    pub transform: Transform,
}
