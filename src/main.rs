use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use player::*;

pub mod player;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugin(RapierDebugRenderPlugin::default());
    app.add_system(grab_mouse);
    app.add_startup_system(create_world);
    app.add_startup_system(add_player);
    app.add_system(player::input::move_player);
	app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    app.run();
}

fn grab_mouse(
    mut windows: ResMut<Windows>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();
    if mouse.just_pressed(MouseButton::Left) {
        //window.set_cursor_visibility(false);
        window.set_cursor_lock_mode(true);
    }
    if key.just_pressed(KeyCode::Escape) {
        //window.set_cursor_visibility(true);
        window.set_cursor_lock_mode(false);
    }
}

fn create_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // floor
    commands.spawn()
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(50.0, 0.1, 50.0))
    .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -10.0, 0.0)))
    .insert_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        material: materials.add(Color::rgb(0.8, 0.0, 0.8).into()),
        ..default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });
}