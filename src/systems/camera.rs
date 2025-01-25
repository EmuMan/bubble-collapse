use bevy::prelude::*;

pub fn spawn_camera(
    mut commands: Commands,
) {
    let camera_pos = Vec3::new(0.0, 0.0, 10.0);
    let camera_transform = Transform::from_translation(camera_pos);
    commands.spawn((Camera2d::default(), camera_transform));
}
