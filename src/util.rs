use bevy::prelude::*;

pub fn continuous_circle_collision(
    pos1: Vec2, vel1: Vec2, radius1: f32,
    pos2: Vec2, vel2: Vec2, radius2: f32,
    delta_time: f32
) -> bool {
    // change frame of reference to account for velocities
    let vel1 = vel1 - vel2;

    // find the closest point on the line representing the path of the first circle
    let d = closest_point_on_line_segment(pos1, pos1 + vel1 * delta_time, pos2);

    // check if the closest point is inside the circle
    let closest_dist_sq = (pos2.x - d.x).powi(2) + (pos2.y - d.y).powi(2);
    let sum_radii = radius1 + radius2;

    closest_dist_sq < sum_radii.powi(2)
}

pub fn closest_point_on_line_segment(l1: Vec2, l2: Vec2, p: Vec2) -> Vec2 {
    // Direction vector of the segment
    let segment = l2 - l1;

    // Squared length of the segment
    let segment_length_squared = segment.length_squared();

    // If the segment length is 0, return None
    if segment_length_squared == 0.0 {
        return l1; // The segment is just a point
    }

    // Project point `p` onto the infinite line defined by `l1` and `l2`
    let t = (p - l1).dot(segment) / segment_length_squared;

    // Confine t to the line segment
    let t = t.clamp(0.0, 1.0);

    // Compute the closest point on the segment
    l1 + segment * t
}
