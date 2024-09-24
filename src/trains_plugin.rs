use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;
use bevy_rts_camera::{RtsCamera, RtsCameraControls, RtsCameraPlugin};

pub struct TrainsPlugin;

impl Plugin for TrainsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RtsCameraPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, draw_cursor);
    }
}

#[derive(Component)]
struct Terrain;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //mesh
    commands.spawn((
        SceneBundle {
            scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset(
                "../assets/models/Peachs Castle Exterior/gltf/flipped_faces/untitled.glb",
            )),
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
    terrains: Query<&GlobalTransform, With<Terrain>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
    mut raycast: Raycast,
) {
    let (camera, camera_transform) = cameras.single();
    let terr = terrains.single();
    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };
    let intersections = raycast.debug_cast_ray(ray, &default(), &mut gizmos);
    let point: Vec3 = match intersections.len() > 0 {
        true => intersections[0].1.position(),
        false => Vec3::ZERO,
    };

    gizmos.circle(point + terr.up() * 0.01, terr.up(), 0.2, Color::WHITE);
}
