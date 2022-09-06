use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub mod input;

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
        camera_cmds.insert(PlayerCamera { body: body_id });
        camera_cmds.insert_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 6.0, 0.0),
            ..default()
        });

        camera_cmds.id()
    });

    body.add_child(camera);
    body.insert(PlayerBody { camera });
}

/// A component defining the player body
#[derive(Component)]
pub struct PlayerBody {
    pub camera: Entity
}

/// A component defining the player camera
#[derive(Component)]
pub struct PlayerCamera {
    pub body: Entity
}