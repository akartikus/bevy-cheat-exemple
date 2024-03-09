use bevy::prelude::*;

use crate::Player;

const CAMERA_DISTANCE: Vec3 = Vec3::new(0.0, 4.5, -9.0);
pub struct CameraPlugin;

#[derive(Component)]

struct MyCamera;

#[derive(Bundle)]
struct CameraBunble {
    camera_3d: Camera3dBundle,
    my_camera: MyCamera,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_follow_system);
    }
}
fn setup_camera(mut commands: Commands) {
    commands.spawn(CameraBunble {
        camera_3d: Camera3dBundle {
            transform: Transform::from_translation(CAMERA_DISTANCE).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        my_camera: MyCamera,
    });
}

fn camera_follow_system(
    mut query: Query<&mut Transform, With<MyCamera>>,
    player_query: Query<&GlobalTransform, With<Player>>,
) {
    if let Ok(mut camera_transform) = query.get_single_mut() {
        if let Ok(player_global_transform) = player_query.get_single() {
            let player_position = player_global_transform.translation();
            let desired_position = player_position + CAMERA_DISTANCE;

            camera_transform.translation = desired_position;
        }
    }
}
