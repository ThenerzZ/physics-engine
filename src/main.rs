use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
mod ui;
use ui::UiPlugin;

#[derive(Component)]
struct OrbitCamera {
    focus: Vec3,
    radius: f32,
    rotate_sensitivity: f32,
    zoom_sensitivity: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            focus: Vec3::ZERO,
            radius: 10.0,
            rotate_sensitivity: 1.0,
            zoom_sensitivity: 0.8,
        }
    }
}

fn orbit_camera(
    mut query: Query<(&mut Transform, &mut OrbitCamera)>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
) {
    let mut rotation_move = Vec2::ZERO;
    let mut scroll: f32 = 0.0;

    // Handle mouse input
    if mouse_buttons.pressed(MouseButton::Right) {
        for ev in mouse_motion.read() {
            rotation_move += ev.delta;
        }
    }
    
    for ev in mouse_wheel.read() {
        scroll += ev.y;
    }

    for (mut transform, mut camera) in query.iter_mut() {
        if rotation_move.length_squared() > 0.0 {
            let delta_x = rotation_move.x / 180.0 * std::f32::consts::PI * camera.rotate_sensitivity;
            let delta_y = rotation_move.y / 180.0 * std::f32::consts::PI * camera.rotate_sensitivity;
            
            // Orbit around focus point
            let mut position = transform.translation - camera.focus;
            
            // Rotate around Y axis
            let rot_matrix = Mat3::from_rotation_y(-delta_x);
            position = rot_matrix * position;
            
            // Rotate around local X axis
            let right = transform.right();
            let rot_matrix = Mat3::from_axis_angle(right, -delta_y);
            position = rot_matrix * position;
            
            transform.translation = position + camera.focus;
            transform.look_at(camera.focus, Vec3::Y);
        }

        // Zoom
        if scroll.abs() > 0.0 {
            let zoom_factor = 1.0 - scroll * camera.zoom_sensitivity;
            camera.radius = (camera.radius * zoom_factor).max(2.0).min(20.0);
            
            let forward = transform.forward();
            transform.translation = camera.focus - forward * camera.radius;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(UiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, orbit_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-5.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        OrbitCamera::default(),
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Ground plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(10.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(5.0, 0.1, 5.0),
    ));
}
