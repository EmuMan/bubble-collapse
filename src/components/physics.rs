use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Collider {
    pub radius: f32,
}

impl Collider {
    pub fn is_point_inside(&self, pos: Vec2, point: Vec2) -> bool {
        pos.distance_squared(point) < self.radius * self.radius
    }
}

#[derive(Component, Debug, Default)]
pub struct Velocity {
    pub velocity: Vec2,
}
