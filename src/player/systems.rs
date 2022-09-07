use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_egui::{egui, EguiContext};
use bevy_rapier3d::prelude::*;

use super::components::{PlayerBody, PlayerCamera};

// Camera vertical look restrictions
// (so you can't snap your own spine)
const CAMERA_LIMIT: f32 = 0.05235988;

// Sets movement information
pub fn player_character_input(
    mut windows: ResMut<Windows>,
    mut mouse_events: EventReader<MouseMotion>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_body_query: Query<&mut PlayerBody>
) {
    // Figure out what direction the player wants to go
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

    // Normalise move intent so you can't go faster by holding A and W
    move_intent = move_intent.normalize();

    // Get the look intent from mouse movement
    let mut look_intent = Vec2::ZERO;
    let window = windows.get_primary_mut().unwrap();
    if window.is_focused() {
        for ev in mouse_events.iter() {
            look_intent += ev.delta;
        }
    }
    
    // Apply changes
    let mut player_body = player_body_query.get_single_mut().unwrap();
    player_body.current_move_intent = move_intent;
    player_body.current_look_intent = look_intent;
}

pub fn player_character_move(
    mut commands: Commands,
    mut time: Res<Time>,
    mut player_body_query: Query<(Entity, &mut PlayerBody, &mut Transform, &mut ExternalImpulse), Without<PlayerCamera>>,
    mut player_camera_query: Query<(Entity, &mut PlayerCamera, &mut Transform), Without<PlayerBody>>
) {
    // Queries
    let mut player_body = player_body_query.get_single_mut().unwrap();
    let mut player_camera = player_camera_query.get_single_mut().unwrap();

    // Turn the player
    let mut bodycomp = player_body.1;
    let mut turn_delta = bodycomp.current_look_intent * time.delta_seconds() * 0.01;
    turn_delta.x *= bodycomp.h_turn_speed;
    turn_delta.y *= bodycomp.v_turn_speed;
    bodycomp.current_look_horizontal = bodycomp.current_look_horizontal - turn_delta.x;
    bodycomp.current_look_vertical = (bodycomp.current_look_vertical - turn_delta.y).clamp(-FRAC_PI_2 + CAMERA_LIMIT, FRAC_PI_2 - CAMERA_LIMIT);
    player_body.2.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, bodycomp.current_look_horizontal, 0.0);
    player_camera.2.rotation = Quat::from_euler(EulerRot::XYZ, bodycomp.current_look_vertical, 0.0, 0.0);
}