use bevy::prelude::*;
use bevy_xpbd_3d::{components::RigidBody, plugins::collision::Collider};

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_npc);
    }
}

fn spawn_npc(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 0.5)),
            material: materials.add(Color::rgb(1.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 0.5, 2.0),
            ..default()
        },
    ));
}
