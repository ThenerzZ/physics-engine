use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod editor;
mod physics;
mod ui;

use editor::{EditorPlugin, EditorCamera};
use physics::PhysicsPlugin;
use ui::UiPlugin;

// Scene Colors
const GROUND_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
const OBJECT_COLOR: Color = Color::rgb(1.0, 0.4, 0.0);
const CLEAR_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1280., 720.).into(),
                    title: "Physics Engine".to_string(),
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            EditorPlugin,
            PhysicsPlugin,
            UiPlugin,
        ))
        .insert_resource(ClearColor(CLEAR_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            editor_controls,
            handle_physics_interactions,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D Camera with viewport rect to avoid UI
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                // Start after the UI panel (280px + padding)
                viewport: Some(bevy::render::camera::Viewport {
                    physical_position: UVec2::new(300, 0),
                    physical_size: UVec2::new(980, 720),  // 1280 - 300 = 980
                    ..default()
                }),
                ..default()
            },
            ..default()
        },
        EditorCamera,
    ));

    // UI Camera
    commands.spawn(Camera2dBundle::default());

    // Increase light intensity
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,  // Double the intensity
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Increase ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,  // Increase brightness
    });

    // Ground plane with brighter material
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(10.0).into()),
            material: materials.add(StandardMaterial {
                base_color: GROUND_COLOR,
                perceptual_roughness: 0.9,
                metallic: 0.0,  // No metallic
                reflectance: 0.1,  // Low reflectance
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(5.0, 0.0, 5.0),
    ));

    // Example physics object with brighter material
    spawn_cube(&mut commands, &mut meshes, &mut materials, Vec3::new(0.0, 3.0, 0.0));
}

fn spawn_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube::new(1.0).into()),
            material: materials.add(StandardMaterial {
                base_color: OBJECT_COLOR,
                perceptual_roughness: 0.7,
                metallic: 0.0,  // No metallic
                reflectance: 0.1,  // Low reflectance
                ..default()
            }),
            transform: Transform::from_translation(position),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
    ));
}

fn editor_controls(
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        spawn_cube(
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3::new(0.0, 3.0, 0.0),
        );
    }
}

fn handle_physics_interactions() {
    // Will be implemented later
}