use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_rapier3d::prelude::*;

use super::{PlayerBody, PlayerCamera};

const SPEED: f32 = 500.0;

pub fn move_player(
    mut commands: Commands,
    mut player_bodies: Query<(Entity, &mut PlayerBody, &mut Transform, &mut ExternalImpulse), Without<PlayerCamera>>,
    mut player_cameras: Query<(Entity, &mut PlayerCamera, &mut Transform), Without<PlayerBody>>,
    mut time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    mut keyboard_input: Res<Input<KeyCode>>,
) {
    let delta = time.delta_seconds();

    let mut move_intent = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::W) {
        move_intent.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        move_intent.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        move_intent.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        move_intent.x -= 1.0;
    }
    move_intent = move_intent.normalize();

    for (entity, mut body, mut transform, mut impulse) in &mut player_bodies {
        for ev in mouse_events.iter() {
            transform.rotate_z(ev.delta.x * 50.0 * delta);
        }
        
        let move_delta = Vec3::new(
            move_intent.x * delta * SPEED,
            move_intent.y * delta * SPEED,
            move_intent.z * delta * SPEED
        );
        impulse.impulse = move_delta;

        match player_cameras.get_mut(body.camera) {
            Ok(mut camera_query) => {
                for ev in mouse_events.iter() {
                    camera_query.2.rotate_local_y(ev.delta.y * 50.0 * delta);
                }
            },
            Err(error) => {
                error!("Player camera was invalid: {}", error);
            },
        }
    }
}