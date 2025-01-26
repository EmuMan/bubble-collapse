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
    for event in bubble_destroyed_event.read() {
        let mut shockwave_color = event.color.clone();
        shockwave_color.set_alpha(0.3);
        let material = materials.add(shockwave_color);
        commands.spawn(BubbleShockwaveBundle {
            mesh: Mesh2d(meshes.add(Circle::new(event.radius))),
            mesh_material: MeshMaterial2d(material),
            transform: Transform::from_translation(event.position.extend(0.0)),
            bubble_shockwave: BubbleShockwave::new(event.radius, 250.0, 250.0, 1.0, true),
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
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut shockwave_query: Query<(Entity, &mut BubbleShockwave, &mut Collider, &Mesh2d, &MeshMaterial2d<ColorMaterial>)>,
) {
    for (entity, mut shockwave, mut collider, mesh, material) in shockwave_query.iter_mut() {
        if let Some(shockwave_time) = shockwave.tick(time.delta().as_secs_f32()) {
            collider.radius = shockwave.radius;
            meshes.insert(mesh, Circle::new(shockwave.radius).into());
            materials.get_mut(material).map(|mat| {
                mat.color.set_alpha((1.0 - shockwave_time).powf(0.5) * 0.3);
            });
        } else {
            commands.entity(entity).despawn();
        }
    }
}
