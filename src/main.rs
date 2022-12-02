#![allow(unused)]

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;

#[derive(Component)]
struct Shape;

#[derive(Component)]
struct StatsText;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                })
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Bevy playground!".to_string(),
                        width: 1920.,
                        height: 1080.,
                        monitor: MonitorSelection::Index(1),
                        position: WindowPosition::Centered,
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    },
                    ..default()
                }),
        )
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(fps_system)
        .add_system(rotate_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::rgb(0.9, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Shape);
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-0.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands
        .spawn(
            TextBundle::from_sections([TextSection::new(
                "\nfps: ",
                TextStyle {
                    font: asset_server.load("fonts/roboto_mono_for_powerline.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(1.0, 1.0, 1.0),
                },
            )])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(StatsText);
}

fn fps_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<StatsText>>) {
    let mut text = query.single_mut();

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            text.sections[0].value = format!("{average:.2} fps");
        }
    };
}

fn rotate_system(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}
