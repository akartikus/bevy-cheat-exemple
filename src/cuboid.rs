use bevy::prelude::*;

use crate::{movement::Velocity, Player};

const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, -1.0);

#[derive(Bundle)]
struct CuboidBundle {
    velocity: Velocity,
    model: PbrBundle,
    player: Player,
}

pub struct CuboidPlugin;

impl Plugin for CuboidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cuboid);
    }
}

fn spawn_cuboid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(CuboidBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        model: PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.5)),
            material: materials.add(Color::rgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        player: Player,
    });
}
