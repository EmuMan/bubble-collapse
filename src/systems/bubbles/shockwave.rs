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
        match event.bubble_type {
            BubbleType::Normal => {
                spawn_normal_shockwave(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    event.position,
                    event.radius,
                    event.color,
                );
            }
            BubbleType::Mega => {
                spawn_mega_shockwave(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    event.position,
                    event.radius,
                    event.color,
                );
            }
            BubbleType::ScatterShot => {}
            BubbleType::Beam => {}
            BubbleType::BlackHole => {}
        }
    }
}

fn spawn_normal_shockwave(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    position: Vec2,
    radius: f32,
    color: Color,
) {
    let mut shockwave_color = color.clone();
    shockwave_color.set_alpha(0.3);
    commands.spawn(BubbleShockwaveBundle {
        mesh: Mesh2d(meshes.add(Circle::new(radius))),
        mesh_material: MeshMaterial2d(materials.add(shockwave_color)),
        transform: Transform::from_translation(position.extend(0.0)),
        bubble_shockwave: BubbleShockwave::new(radius, 250.0, 50.0, 1.0, true),
        collider: Collider {
            radius,
            ..Default::default()
        },
    });
}

fn spawn_mega_shockwave(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    position: Vec2,
    radius: f32,
    color: Color,
) {
    let mut shockwave_color = color.clone();
    shockwave_color.set_alpha(0.5);
    commands.spawn(BubbleShockwaveBundle {
        mesh: Mesh2d(meshes.add(Circle::new(radius))),
        mesh_material: MeshMaterial2d(materials.add(shockwave_color)),
        transform: Transform::from_translation(position.extend(0.0)),
        bubble_shockwave: BubbleShockwave::new(radius, 500.0, 750.0, 100.0, true),
        collider: Collider {
            radius,
            ..Default::default()
        },
    });
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
