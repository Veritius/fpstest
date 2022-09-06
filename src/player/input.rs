use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_rapier3d::prelude::*;

use super::{PlayerBody, PlayerCamera};

const MOVE_SPEED: f32 = 500.0;
const LOOK_SPEED: f32 = 500.0;

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
		println!("move up");
    }
    if keyboard_input.pressed(KeyCode::S) {
        move_intent.z -= 1.0;
		println!("move down");
    }
    if keyboard_input.pressed(KeyCode::D) {
        move_intent.x += 1.0;
		println!("move right");
    }
    if keyboard_input.pressed(KeyCode::A) {
        move_intent.x -= 1.0;
		println!("move left");
    }
    move_intent = move_intent.normalize();

    for (entity, mut body, mut transform, mut impulse) in &mut player_bodies {
        for ev in mouse_events.iter() {
            transform.rotate_local_z(ev.delta.x * delta * LOOK_SPEED);
        }
        
        let move_delta = Vec3::new(
            move_intent.x * delta * MOVE_SPEED,
            move_intent.y * delta * MOVE_SPEED,
            move_intent.z * delta * MOVE_SPEED
        );
        impulse.impulse = move_delta;

        match player_cameras.get_mut(body.camera) {
            Ok(mut camera_query) => {
                for ev in mouse_events.iter() {
                    camera_query.2.rotate_local_y(ev.delta.y * delta * LOOK_SPEED);
                }
            },
            Err(error) => {
                error!("Player body {:?}'s camera {:?} is invalid: {}", entity, body.camera, error);
            },
        }
    }
}