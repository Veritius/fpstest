use bevy::{prelude::{Component, Entity, KeyCode}, math::{Vec3, Vec2, Quat}};
use bevy_egui::egui::Key;

/// A component defining the player body
#[derive(Component)]
pub struct PlayerBody {
    pub is_running: bool,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub h_turn_speed: f32,
    pub v_turn_speed: f32,

    pub current_look_horizontal: f32,
    pub current_look_vertical: f32,
    pub current_move_intent: Vec3,
    pub current_look_intent: Vec2
}

/// A component defining the player camera
#[derive(Component)]
pub struct PlayerCamera;