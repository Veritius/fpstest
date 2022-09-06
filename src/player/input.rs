use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_egui::{egui, EguiContext};
use bevy_rapier3d::prelude::*;

use super::{PlayerBody, PlayerCamera};

const MOVE_SPEED: f32 = 500.0;
const LOOK_SPEED: f32 = 20.0;

pub fn move_player(
    mut commands: Commands,
    mut player_bodies: Query<(Entity, &mut PlayerBody, &mut Transform, &mut ExternalImpulse), Without<PlayerCamera>>,
    mut player_cameras: Query<(Entity, &mut PlayerCamera, &mut Transform), Without<PlayerBody>>,
    mut time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    mut keyboard_input: Res<Input<KeyCode>>,
    mut egui_context: ResMut<EguiContext>
) {
    // I'll write a macro for this later
    let mut dmsg = Vec::<String>::new();

    // Delta time, to adjust for tickrate so players m·ove in real time
    let delta = time.delta_seconds();
    dmsg.push(format!("Time delta: {}", delta));

    // Get move intentEguiContext;
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
    dmsg.push(format!("Movement intent: {}", move_intent));

    // Move delta for physics impulse
    let move_delta = Vec3::new(
        move_intent.x * delta * MOVE_SPEED,
        move_intent.y * delta * MOVE_SPEED,
        move_intent.z * delta * MOVE_SPEED
    );
    dmsg.push(format!("Movement delta: {}", move_delta));

    // Get the look intent from mouse movement
    let mut look_intent = Vec2::ZERO;
    for ev in mouse_events.iter() {
        look_intent += ev.delta;
    }

    let mut look_delta = look_intent * delta * LOOK_SPEED;
    dmsg.push(format!("Look intent: {}", look_intent));
    dmsg.push(format!("Look delta: {}", look_delta));

    // Transform player bodies (you should never have more than one but fucked if I care)
    for (entity, mut body, mut transform, mut impulse) in &mut player_bodies {
        // Rotate the body
        transform.rotate_local_z(look_intent.x);
        
        // Physics impulse to move the body
        impulse.impulse = move_delta;

        // Transform each body's camera
        match player_cameras.get_mut(body.camera) {
            Ok(mut camera_query) => {
                // Rotate the camera
                camera_query.2.rotate_local_y(look_intent.y);

                dmsg.push(format!("Player camera position: {}", camera_query.2.translation));
                dmsg.push(format!("Player camera rotation: {}", camera_query.2.rotation));
            },
            Err(error) => {
                // uh oh
                error!("Player body {:?}'s camera {:?} is invalid: {}", entity, body.camera, error);
            },
        }

        dmsg.push(format!("Player body position: {}", transform.translation));
        dmsg.push(format!("Player body rotation: {}", transform.rotation));
    }

    egui::Window::new("Debug information").show(egui_context.ctx_mut(), |ui| {
        for msg in dmsg {
            ui.label(msg);
        }
    });
}