//! This example is set up for debugging. It draws gizmos to indicate various aspects, and you
//! can toggle between the RTS camera view and an orbit camera in order to see a different
//! perspective and how the camera behaves in different circumstances.
//!
//!    Space: toggle camera
//!    L: toggle lock onto target

use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use bevy_rts_camera::{
    CameraEye, CameraPivot, CameraState, Ground, RtsCameraPlugin, RtsCameraSystemSet,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RtsCameraPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                animate_unit,
                (lock_or_jump, swap_cameras, snap_to_location),
                debug,
            )
                .chain()
                .before(RtsCameraSystemSet),
        )
        .run();
}

#[derive(Component)]
struct Move;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(15.0, 15.0)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        })
        .insert(Ground);
    // Some "terrain"
    let terrain_material = materials.add(Color::rgb(0.8, 0.7, 0.6));
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 0.5, 1.0)),
            material: terrain_material.clone(),
            transform: Transform::from_xyz(0.0, 0.25, 0.0),
            ..default()
        })
        .insert(Ground);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(3.0, 0.2, 1.0)),
            material: terrain_material.clone(),
            transform: Transform::from_xyz(3.0, 0.1, -1.0),
            ..default()
        })
        .insert(Ground);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(2.0, 0.3, 3.0)),
            material: terrain_material.clone(),
            transform: Transform::from_xyz(-3.0, 0.15, 0.0),
            ..default()
        })
        .insert(Ground);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Sphere::new(3.0)),
            material: terrain_material.clone(),
            transform: Transform::from_xyz(-5.0, 0.0, 3.0),
            ..default()
        })
        .insert(Ground);
    // A moving unit
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.1, 0.3)),
            material: terrain_material.clone(),
            transform: Transform::from_xyz(0.0, 0.25, 0.0),
            ..default()
        })
        .insert(Move);
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // Camera
    commands
        .spawn((
            TransformBundle {
                local: Transform::from_translation(Vec3::new(0.0, 5.5, 5.5)),
                ..default()
            },
            CameraPivot,
        ))
        .with_children(|parent| {
            parent.spawn((Camera3dBundle::default(), CameraEye));
        });
    // Debug Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(8.0, 1.0, 14.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                is_active: false,
                ..default()
            },
            ..default()
        },
        PanOrbitCamera {
            enabled: false,
            zoom_sensitivity: 0.0,
            ..default()
        },
    ));
}

/// Move the cube in a circle around the Y axis
fn animate_unit(
    time: Res<Time>,
    mut cube_q: Query<&mut Transform, With<Move>>,
    mut angle: Local<f32>,
) {
    if let Ok(mut cube_tfm) = cube_q.get_single_mut() {
        // Rotate 20 degrees a second, wrapping around to 0 after a full rotation
        *angle += 20f32.to_radians() * time.delta_seconds() % TAU;
        // Convert angle to position
        let pos = Vec3::new(angle.sin() * 1.5, 0.25, angle.cos() * 1.5);
        cube_tfm.translation = pos;
    }
}

fn lock_or_jump(
    mut camera_state: ResMut<CameraState>,
    key_input: Res<ButtonInput<KeyCode>>,
    cube_q: Query<&Transform, With<Move>>,
) {
    for cube in cube_q.iter() {
        if key_input.pressed(KeyCode::KeyL) {
            camera_state.snap_to(cube.translation);
        }
        if key_input.just_pressed(KeyCode::KeyK) {
            camera_state.jump_to(cube.translation);
        }
    }
}

fn snap_to_location(
    mut camera_state: ResMut<CameraState>,
    button_input: Res<ButtonInput<KeyCode>>,
) {
    if button_input.just_pressed(KeyCode::KeyT) {
        camera_state.snap_to(Vec3::new(3.0, 13.0, 3.0));
    }
}

fn swap_cameras(
    mut orbit_cam: Query<(&mut Camera, &mut PanOrbitCamera)>,
    mut rts_cam: Query<&mut Camera, (With<CameraEye>, Without<PanOrbitCamera>)>,
    button_input: Res<ButtonInput<KeyCode>>,
) {
    if button_input.just_pressed(KeyCode::Space) {
        let (mut orbit_camera, mut orbit_cam) = orbit_cam.get_single_mut().unwrap();
        let mut rts_cam = rts_cam.get_single_mut().unwrap();
        orbit_camera.is_active = !orbit_camera.is_active;
        orbit_cam.enabled = orbit_camera.is_active;
        rts_cam.is_active = !rts_cam.is_active;
    }
}

fn debug(
    rts_camera: Query<(&GlobalTransform, &CameraPivot), Without<CameraEye>>,
    rts_cam_eye: Query<&mut GlobalTransform, With<CameraEye>>,
    mut gizmos: Gizmos,
) {
    for (rts_cam_tfm, _) in rts_camera.iter() {
        gizmos.ray(
            rts_cam_tfm.translation(),
            rts_cam_tfm.forward(),
            Color::AQUAMARINE,
        );
        gizmos.ray(rts_cam_tfm.translation(), rts_cam_tfm.back(), Color::BLUE);
        gizmos.ray(rts_cam_tfm.translation(), rts_cam_tfm.up(), Color::GREEN);
        gizmos.ray(rts_cam_tfm.translation(), rts_cam_tfm.right(), Color::RED);
    }

    for eye_tfm in rts_cam_eye.iter() {
        gizmos.sphere(eye_tfm.translation(), Quat::IDENTITY, 0.2, Color::PURPLE);
        gizmos.arrow(
            eye_tfm.translation(),
            eye_tfm.translation() + eye_tfm.forward(),
            Color::PINK,
        );
    }
}
