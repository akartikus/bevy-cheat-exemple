use bevy::{input::keyboard, log::tracing_subscriber::fmt::time, prelude::*, utils::info};
use bevy_xpbd_3d::{components::LinearVelocity, plugins::spatial_query::ShapeHits};

use crate::Player;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}

// fn update_position(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut query: Query<&mut Transform, With<Player>>,
//     time: Res<Time>,
// ) {
//     let mut cuboid_transform = query.single_mut();
//     let mut velocity = Vec3::ZERO;
//     if keyboard_input.pressed(KeyCode::ArrowLeft) {
//         velocity = Vec3::X;
//     }
//     if keyboard_input.pressed(KeyCode::ArrowRight) {
//         velocity = Vec3::NEG_X;
//     }
//     if keyboard_input.pressed(KeyCode::ArrowUp) {
//         velocity = Vec3::Z;
//     }
//     if keyboard_input.pressed(KeyCode::ArrowDown) {
//         velocity = Vec3::NEG_Z;
//     }
//     if velocity.length() > 0.0 {
//         velocity = velocity.normalize();
//     }
//     if velocity.length_squared() > 0.0 {
//         // TODO make look to smooth, use rotation?
//         cuboid_transform.look_to(velocity, Vec3::Y);
//     }
//     cuboid_transform.translation += velocity * time.delta_seconds();
// }

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&mut LinearVelocity, &mut Transform, &ShapeHits), With<Player>>,
    time: Res<Time>,
) {
    for (mut linear_velocity, mut cuboid_transform, ground_hits) in &mut players {
        let mut velocity = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            // linear_velocity.z -= 12;
            velocity = Vec3::NEG_Z;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            // linear_velocity.z += 1.2;
            velocity = Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            // linear_velocity.x -= 1.2;
            velocity = Vec3::NEG_X;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            // linear_velocity.x += 1.2;
            velocity = Vec3::X;
        }
        if velocity.length() > 0.0 {
            velocity = velocity.normalize();
        }
        linear_velocity.x = velocity.x * 150.0 * time.delta_seconds();
        linear_velocity.z = velocity.z * 150.0 * time.delta_seconds();

        if keyboard_input.pressed(KeyCode::Space) && !ground_hits.is_empty() {
            linear_velocity.y += 15.0 * time.delta_seconds();
            // Slow player down on the x and y axes
            linear_velocity.x *= 0.8;
            linear_velocity.z *= 0.8;
        }
        if velocity.length_squared() > 0.0 {
            cuboid_transform.look_to(velocity, Vec3::Y);
        }
    }
}
