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
    time: Res<Time>,
    physics_context: Res<RapierContext>,
    mut player_body_query: Query<(Entity, &mut PlayerBody, &mut Transform, &mut Velocity), Without<PlayerCamera>>,
    mut player_camera_query: Query<(Entity, &mut PlayerCamera, &mut Transform), Without<PlayerBody>>,
    mut uicontext: ResMut<EguiContext>
) {
    // Queries
    let mut player_body = player_body_query.get_single_mut().unwrap();
    let mut player_camera = player_camera_query.get_single_mut().unwrap();

    // Turn the player
    let mut bodycomp = player_body.1;
    let mut turn_delta = bodycomp.current_look_intent * time.delta_seconds() * 0.01 * bodycomp.move_speed_modifier;
    turn_delta.x *= bodycomp.h_turn_speed;
    turn_delta.y *= bodycomp.v_turn_speed;
    bodycomp.current_look_horizontal = bodycomp.current_look_horizontal - turn_delta.x;
    bodycomp.current_look_vertical = (bodycomp.current_look_vertical - turn_delta.y).clamp(-FRAC_PI_2 + CAMERA_LIMIT, FRAC_PI_2 - CAMERA_LIMIT);
    player_body.2.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, bodycomp.current_look_horizontal, 0.0);
    player_camera.2.rotation = Quat::from_euler(EulerRot::XYZ, bodycomp.current_look_vertical, 0.0, 0.0);

    // Move the player (code taken from https://github.com/qhdwight/bevy_fps_controller)
    let original_velocity = player_body.3.linvel;
    let mut ground_hit = None;
    let cast_capsule = Collider::capsule_y(
        1.0,
        1.0,
    );
    let cast_velocity = Vec3::NEG_Y;
    let max_distance = 0.125;
    let groups = QueryFilter::default().exclude_rigid_body(player_body.0);
    let orientation = Quat::from_euler(EulerRot::XYZ, bodycomp.current_look_vertical, bodycomp.current_look_horizontal, 0.0);
    if let Some((_handle, hit)) = physics_context.cast_shape(
        player_body.2.translation,
        orientation,
        cast_velocity,
        &cast_capsule,
        max_distance,
        groups,
    ) {
        ground_hit = Some(hit);
    }

    // If touching the ground, allow movement
    match ground_hit {
        Some(toi) => {
            let mut movespeed = bodycomp.move_speed_modifier;
            if bodycomp.is_running {
                movespeed *= bodycomp.run_speed;
            } else {
                movespeed *= bodycomp.walk_speed;
            }
            let forward = orientation * Vec3::X;
            let right = orientation * Vec3::NEG_Z;
            let velocity_mod = bodycomp.current_move_intent.x * forward + bodycomp.current_move_intent.y * right;
            player_body.3.linvel = original_velocity + (velocity_mod * movespeed);
        },
        None => {},
    }
    
    egui::Window::new("Debug").show(uicontext.ctx_mut(), |ui| {
        ui.label(format!("Body position: {}", player_body.2.translation));
        ui.label(format!("Body rotation: {}", player_body.2.rotation));
        ui.label(format!("Head position: {}", player_camera.2.translation));
        ui.label(format!("Head rotation: {}", player_camera.2.rotation));
    });
}