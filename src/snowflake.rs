use std::i32;

use bevy::{color::palettes::css::*, prelude::*};

pub struct SnowflakePlugin;

impl Plugin for SnowflakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, setup_ui))
            .add_systems(Update, (draw, update_ui));
    }
}

#[derive(Resource)]
struct LevelsAndDistance {
    levels: i32,
    dist_coeff: f32,
}

#[derive(Component)]
struct LevelsAndDistanceDisplayBg;

#[derive(Component)]
struct LevelsAndDistanceDisplayText;

fn setup_ui(mut commands: Commands) {
    let root = commands
        .spawn((
            LevelsAndDistanceDisplayBg,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_alpha(0.5)),
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    top: Val::Percent(1.),
                    left: Val::Percent(1.),
                    right: Val::Auto,
                    bottom: Val::Auto,
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let text = commands
        .spawn((
            LevelsAndDistanceDisplayText,
            TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: "Levels: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: "Dist: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();

    commands.entity(root).push_children(&[text]);

    // commands
    //     .spawn(NodeBundle {
    //         style: Style {
    //             display: Display::Flex,                     // Use Flexbox for layout
    //             flex_direction: FlexDirection::Column,      // Stack children vertically
    //             justify_content: JustifyContent::FlexStart, // Align items at the start
    //             align_items: AlignItems::Center,            // Center items horizontally
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     })
    //     .with_children(|parent| {
    //         // Add text sections as children of the column node
    //         parent.spawn(TextBundle {
    //             text: Text::from_section(
    //                 "First Line",
    //                 TextStyle {
    //                     font_size: 30.0,
    //                     color: Color::WHITE,
    //                     ..Default::default()
    //                 },
    //             ),
    //             ..Default::default()
    //         });

    //         parent.spawn(TextBundle {
    //             text: Text::from_section(
    //                 "Second Line",
    //                 TextStyle {
    //                     font_size: 30.0,
    //                     color: Color::WHITE,
    //                     ..Default::default()
    //                 },
    //             ),
    //             ..Default::default()
    //         });

    //         parent.spawn(TextBundle {
    //             text: Text::from_section(
    //                 "Third Line",
    //                 TextStyle {
    //                     font_size: 30.0,
    //                     color: Color::WHITE,
    //                     ..Default::default()
    //                 },
    //             ),
    //             ..Default::default()
    //         });
    //     });
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(LevelsAndDistance {
        levels: 4,
        dist_coeff: 50.0,
    });
}

fn update_ui(
    settings: ResMut<LevelsAndDistance>,
    mut query: Query<&mut Text, With<LevelsAndDistanceDisplayText>>,
) {
    for mut text in &mut query {
        for sec in text.sections.iter_mut() {
            if sec.value.contains("Levels:") {
                sec.value = format!("Levels: {}", settings.levels);
            }
            if sec.value.contains("Dist:") {
                sec.value = format!("Dist: {}", settings.dist_coeff);
            }
        }
    }
}

fn draw(
    mut gizmos: Gizmos,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<LevelsAndDistance>,
) {
    if keyboard.just_pressed(KeyCode::KeyJ) {
        settings.levels += 1;
    }
    if keyboard.just_pressed(KeyCode::KeyK) {
        settings.levels -= 1;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        settings.dist_coeff += 1.;
    }
    if keyboard.pressed(KeyCode::KeyF) {
        settings.dist_coeff -= 1.;
    }

    settings.levels = i32::clamp(settings.levels, 1, 10);
    settings.dist_coeff = f32::clamp(settings.dist_coeff, 1., 100.);

    draw_recursively(
        &mut gizmos,
        settings.levels,
        settings.dist_coeff,
        Vec2::ZERO,
    );
}

fn draw_recursively(gizmos: &mut Gizmos, levels: i32, dist_coeff: f32, center: Vec2) {
    if levels <= 0 {
        return;
    }

    let dist: f32 = levels as f32 * dist_coeff;
    let top: Vec2 = center + Vec2::Y * dist;
    let left: Vec2 = center + Vec2::from_angle(f32::to_radians(90. + 1. * 360. / 3.)) * dist;
    let right: Vec2 = center + Vec2::from_angle(f32::to_radians(90. + 2. * 360. / 3.)) * dist;

    gizmos.line_2d(center, top, RED);
    gizmos.line_2d(center, left, BLUE);
    gizmos.line_2d(center, right, GREEN);

    for v in vec![top, left, right] {
        draw_recursively(gizmos, levels - 1, dist_coeff * 0.75, v);
    }
}
