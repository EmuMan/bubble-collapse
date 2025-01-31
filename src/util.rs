use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component, Resource, Debug, Default, Clone)]
pub struct ActionTimer {
    pub timer: Timer,
    pub amount: u64,
    pub already_completed: u64,
}

impl ActionTimer {
    pub fn new(duration: Duration, amount: u64, mode: TimerMode) -> Self {
        Self {
            timer: Timer::new(duration, mode),
            amount,
            already_completed: 0,
        }
    }

    // Returns how many actions to be completed this tick
    pub fn tick(&mut self, delta: Duration) -> Option<u64> {
        self.timer.tick(delta);
        if self.timer.finished() {
            if self.already_completed >= self.amount {
                self.reset_count_if_repeating();
                None
            } else {
                let left = self.amount - self.already_completed;
                self.already_completed = self.amount;
                self.reset_count_if_repeating();
                Some(left)
            }
        } else {
            let time = self.timer.elapsed_secs() / self.timer.duration().as_secs_f32();
            let expected_actions = (time * self.amount as f32).round() as u64;
            // should never break but just in case...
            let actions = expected_actions.checked_sub(self.already_completed).unwrap_or(0);
            self.already_completed = expected_actions;
            Some(actions)
        }
    }

    fn reset_count_if_repeating(&mut self) {
        if self.timer.mode() == TimerMode::Repeating {
            self.already_completed = 0;
        }
    }

    pub fn reset(&mut self) {
        self.timer.reset();
        self.already_completed = 0;
    }
}

pub fn continuous_circle_collision(
    pos1: Vec2, vel1: Vec2, radius1: f32,
    pos2: Vec2, vel2: Vec2, radius2: f32,
    delta_time: f32
) -> bool {
    circle_close_to_line(
        pos2,
        radius2,
        pos1,
        // Change frame of reference to pos1
        pos1 + (vel1 - vel2) * delta_time,
        radius1 * 2.0,
        true
    )
}

pub fn circle_close_to_line(
    circle_pos: Vec2, circle_radius: f32,
    line_start: Vec2, line_end: Vec2,
    line_width: f32, constrain: bool,
) -> bool {
    // find the closest point on the line representing the path of the first circle
    let d = closest_point_on_line(line_start, line_end, circle_pos, constrain);

    let closest_dist_sq = circle_pos.distance_squared(d);
    let sum_radii = circle_radius + line_width / 2.0;

    closest_dist_sq < sum_radii.powi(2)
}

pub fn closest_point_on_line(l1: Vec2, l2: Vec2, p: Vec2, constrain_to_segment: bool) -> Vec2 {
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
    let t = if constrain_to_segment { t.clamp(0.0, 1.0) } else { t };

    // Compute the closest point on the segment
    l1 + segment * t
}

pub fn spawn_button_with_text(
    commands: &mut Commands,
    text: String,
    width: Val,
    height: Val,
    font_size: f32,
) -> Entity {
    let button_node = Node {
        width,
        height,
        border: UiRect::all(Val::Px(3.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_node = Text::new(text);
    let button_text_color = TextColor(Color::srgb(0.9, 0.9, 0.9));
    let button_text_font = TextFont {
        font_size,
        ..default()
    };

    let button = commands.spawn((
        Button,
        button_node,
        BorderColor(Color::WHITE),
        BackgroundColor(Color::srgb(0.3, 0.5, 0.8)),
    )).id();

    let button_text = commands.spawn((
        button_text_node,
        button_text_color,
        button_text_font
    )).id();

    commands.entity(button).add_children(&[button_text]);

    button
}

pub fn get_viewport_bounds(
    window_query: &Query<&Window, With<PrimaryWindow>>,
    camera_query: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Rect> {
    let min_bounds = Vec2::new(0.0, 0.0);
    let max_bounds = window_query.get_single().ok()?.size();

    let (camera, camera_transform) = camera_query.get_single().ok()?;

    let top_left_world_pos = camera.viewport_to_world_2d(camera_transform, min_bounds).ok()?;
    let bottom_right_world_pos = camera.viewport_to_world_2d(camera_transform, max_bounds).ok()?;

    Some(Rect {
        min: Vec2::new(top_left_world_pos.x, bottom_right_world_pos.y),
        max: Vec2::new(bottom_right_world_pos.x, top_left_world_pos.y),
    })
}

pub fn random_f32(rng: u64, min: f32, max: f32) -> f32 {
    let norm = rng as f32 / u64::MAX as f32;
    min + norm * (max - min)
}
