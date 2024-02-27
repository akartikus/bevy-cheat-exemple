use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::Player;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

fn update_position(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut cuboid_tranform = query.single_mut();
    let mut velocity = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        velocity = Vec3::X;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        velocity = Vec3::NEG_X;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        velocity = Vec3::Z;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        velocity = Vec3::NEG_Z;
    }
    cuboid_tranform.translation += velocity * time.delta_seconds();
}
