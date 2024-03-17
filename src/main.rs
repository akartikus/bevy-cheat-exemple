mod camera;
mod cuboid;
mod movement;

use bevy::prelude::*;
use bevy_xpbd_3d::{
    components::RigidBody,
    plugins::{collision::Collider, PhysicsPlugins},
};
use camera::CameraPlugin;
use cuboid::CuboidPlugin;
use movement::MovementPlugin;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CuboidPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, setup)
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    // commands.spawn((
    //     RigidBody::Static,
    //     Collider::cylinder(0.02, 4.0),
    //     PbrBundle {
    //         mesh: meshes.add(Circle::new(4.0)),
    //         material: materials.add(Color::WHITE),
    //         transform: Transform::from_rotation(Quat::from_rotation_x(
    //             -std::f32::consts::FRAC_PI_2,
    //         )),
    //         ..default()
    //     },
    // ));
    // Plane
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(8.0, 0.002, 8.0),
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(8.0, 8.0)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
