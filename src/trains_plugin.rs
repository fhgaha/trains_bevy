use bevy::{math::VectorSpace, prelude::*};
use bevy_mod_raycast::prelude::*;
use bevy_rts_camera::{RtsCamera, RtsCameraControls, RtsCameraPlugin};

pub struct TrainsPlugin;

impl Plugin for TrainsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RtsCameraPlugin)
            // .add_plugins(CursorRayPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, draw_cursor)
            // .add_systems(Update, raycast)
        ;
    }
}

fn raycast(cursor_ray: Res<CursorRay>, mut raycast: Raycast, mut gizmos: Gizmos) {
    if let Some(cursor_ray) = **cursor_ray {
        raycast.debug_cast_ray(cursor_ray, &default(), &mut gizmos);
    }
}

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Terrain;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // plane
    // commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
    //         material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
    //         ..default()
    //     },
    //     Ground,
    // ));

    //mesh
    commands.spawn((
        SceneBundle {
            scene: asset_server.load(
                GltfAssetLabel::Scene(0)
                    // .from_asset("../assets/models/Peachs Castle Exterior/glb/untitled.glb"),
                    // .from_asset("../assets/models/Peachs Castle Exterior/gltf/untitled.gltf"),
                    .from_asset("../assets/models/Peachs Castle Exterior/gltf/flipped_faces/untitled.glb"),
            ),
            ..default()
        },
        Terrain,
    ));

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle::default(),
        RtsCamera::default(),
        RtsCameraControls::my_controls(),
    ));
}

fn draw_cursor(
    cameras: Query<(&Camera, &GlobalTransform)>,
    // ground_query: Query<&GlobalTransform, With<Ground>>,
    terrains: Query<&GlobalTransform, With<Terrain>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
    mut raycast: Raycast,
) {
    let (camera, camera_transform) = cameras.single();

    let ground = terrains.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let intersections = raycast.debug_cast_ray(ray, &default(), &mut gizmos);

    let mut point = Vec3::ZERO;
    if intersections.len() > 0 {
        point = intersections[0].1.position();
    }
    println!("aaa {point}");
    // Draw a circle just above the ground plane at that position.
    gizmos.circle(point + ground.up() * 0.01, ground.up(), 0.2, Color::WHITE);
    // gizmos.circle(point + Vec3::Y, Dir3::Y, 0.2, Color::WHITE);
}
