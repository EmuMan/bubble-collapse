use bevy::prelude::*;

use crate::components::bubbles::*;
use crate::components::physics::Collider;
use crate::resources::bubbles::*;

pub fn spawn_shockwaves(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut bubble_destroyed_event: EventReader<BubbleDestroyedEvent>,
) {
    let mut shockwave_color = Color::WHITE;
    shockwave_color.set_alpha(0.3);
    for event in bubble_destroyed_event.read() {
        commands.spawn(BubbleShockwaveBundle {
            mesh: Mesh2d(meshes.add(Circle::new(event.radius))),
            mesh_material: MeshMaterial2d(materials.add(shockwave_color)),
            transform: Transform::from_translation(event.position.extend(0.0)),
            bubble_shockwave: BubbleShockwave::new(event.radius, 250.0, 50.0, 1.0, true),
            collider: Collider {
                radius: event.radius,
                ..Default::default()
            },
        });
    }
}

pub fn expand_shockwaves(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shockwave_query: Query<(Entity, &mut BubbleShockwave, &mut Collider, &Mesh2d)>,
) {
    for (entity, mut shockwave, mut collider, mesh) in shockwave_query.iter_mut() {
        if shockwave.tick(time.delta().as_secs_f32()) {
            commands.entity(entity).despawn();
        }
        collider.radius = shockwave.radius;
        meshes.insert(mesh, Circle::new(shockwave.radius).into());
    }
}
