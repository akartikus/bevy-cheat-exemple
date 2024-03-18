use crate::{movement::Velocity, Player};
use bevy::prelude::*;
use bevy_xpbd_3d::{
    components::RigidBody,
    math::{Quaternion, Vector},
    plugins::{collision::Collider, spatial_query::ShapeCaster},
};

const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, -1.0);

#[derive(Bundle)]
struct CuboidBundle {
    velocity: Velocity,
    model: PbrBundle,
    rigid_body: RigidBody,
    collider: Collider,
    shape_caster: ShapeCaster,
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
        // Cast the player shape downwards to detect when the player is grounded
        shape_caster: ShapeCaster::new(
            Collider::capsule(0.9, 0.35),
            Vector::NEG_Y * 0.05,
            Quaternion::default(),
            Direction3d::NEG_Y,
        )
        .with_max_time_of_impact(0.2)
        .with_max_hits(1),
        rigid_body: RigidBody::Dynamic,
        collider: Collider::cuboid(1.0, 1.0, 1.5),
        player: Player,
    });
}
