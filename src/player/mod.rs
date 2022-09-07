use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use self::systems::*;
use self::components::*;

pub mod components;
pub mod systems;

pub struct PlayerCharacterPlugin;

impl Plugin for PlayerCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_player);

        app.add_system(player_character_input);
        app.add_system(player_character_move);
    }
}

pub fn add_player(
    mut commands: Commands
) {
    // Body
    let mut body = commands.spawn();
    let body_id = body.id();

    body.insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 6.0, 0.0)));
    body.insert(RigidBody::Dynamic);
    body.insert(Collider::capsule_y(1.0, 1.0));
    body.insert(Velocity::default());

    // Camera
    let camera = body.add_children(|builder| {
        let mut camera_cmds = builder.spawn();
        camera_cmds.insert(PlayerCamera);
        camera_cmds.insert_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 6.0, 0.0),
            ..default()
        });

        camera_cmds.id()
    });

    body.add_child(camera);
    body.insert(PlayerBody {
        is_running: false,
        walk_speed: 10.0,
        run_speed: 14.0,
        h_turn_speed: 4.0,
        v_turn_speed: 2.0,

        move_speed_modifier: 1.0,
        turn_speed_modifier: 1.0,

        current_move_intent: Vec3::ZERO,
        current_look_intent: Vec2::ZERO,
        current_look_horizontal: 0.0,
        current_look_vertical: 90.0,
    });
}