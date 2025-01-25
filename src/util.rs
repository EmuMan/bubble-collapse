use bevy::prelude::*;

pub fn continuous_circle_collision(
    pos1: Vec2, vel1: Vec2, radius1: f32,
    pos2: Vec2, vel2: Vec2, radius2: f32,
    delta_time: f32
) -> Option<f32> {
    let v_rel = vel2 - vel1;
    let p_rel = pos2 - pos1;
    let combined_radius = radius1 + radius2;

    let a = v_rel.dot(v_rel);
    let b = 2.0 * p_rel.dot(v_rel);
    let c = p_rel.dot(p_rel) - combined_radius.powi(2);

    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        return None; // No collision
    }

    // Calculate roots
    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    // Check for valid time of collision
    if t1 >= 0.0 && t1 <= delta_time {
        return Some(t1); // Collision happens at t1
    }

    if t2 >= 0.0 && t2 <= delta_time {
        return Some(t2); // Collision happens at t2
    }

    None // No collision within this frame
}
