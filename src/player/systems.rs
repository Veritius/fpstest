use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_egui::{egui, EguiContext};
use bevy_rapier3d::prelude::*;

use super::components::{PlayerBody, PlayerCamera};

// Camera vertical look restrictions
// (so you can't snap your own spine)
const CAMERA_LIMIT: f32 = 0.2617994;

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
    move_intent = move_intent.normalize_or_zero();

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
    mut player_body_query: Query<(Entity, &mut PlayerBody, &mut Transform, &mut ExternalImpulse), Without<PlayerCamera>>,
    mut player_camera_query: Query<(Entity, &mut PlayerCamera, &mut Transform), Without<PlayerBody>>,
    mut uicontext: ResMut<EguiContext>
) {
    // Queries
    let (body_id, mut body_comp, mut body_transform, mut body_impulse) = player_body_query.get_single_mut().unwrap();
    let (cam_id, mut cam_comp, mut cam_transform) = player_camera_query.get_single_mut().unwrap();

    // Turn the player
    let mut turn_delta = body_comp.current_look_intent * time.delta_seconds() * 0.01 * body_comp.move_speed_modifier;
    turn_delta.x *= body_comp.h_turn_speed;
    turn_delta.y *= body_comp.v_turn_speed;
    body_comp.current_look_horizontal = body_comp.current_look_horizontal - turn_delta.x;
    body_comp.current_look_vertical = (body_comp.current_look_vertical - turn_delta.y).clamp(-FRAC_PI_2 + CAMERA_LIMIT, FRAC_PI_2 - CAMERA_LIMIT);
    body_transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, body_comp.current_look_horizontal, 0.0);
    cam_transform.rotation = Quat::from_euler(EulerRot::XYZ, body_comp.current_look_vertical, 0.0, 0.0);

    body_impulse.impulse += Vec3::new(
        body_comp.current_move_intent.x * body_comp.move_speed_modifier * 100.0,
        0.0,
        body_comp.current_move_intent.y * body_comp.move_speed_modifier * 100.0
    );
    
    egui::Window::new("Debug").show(uicontext.ctx_mut(), |ui| {
        ui.label(format!("Body position: {}", body_transform.translation));
        ui.label(format!("Body rotation: {}", body_transform.rotation));
        ui.label(format!("Head position: {}", cam_transform.translation));
        ui.label(format!("Head rotation: {}", cam_transform.rotation));
    });
}