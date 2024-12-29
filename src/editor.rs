use bevy::prelude::*;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, editor_camera_controls);
    }
}

#[derive(Component)]
pub struct EditorCamera;

fn editor_camera_controls(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<EditorCamera>>,
) {
    let mut camera_transform = query.single_mut();
    let mut movement = Vec3::ZERO;
    let speed = 5.0;

    if keyboard.pressed(KeyCode::W) {
        movement.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::S) {
        movement.z += 1.0;
    }
    if keyboard.pressed(KeyCode::A) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::D) {
        movement.x += 1.0;
    }
    if keyboard.pressed(KeyCode::Q) {
        movement.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::E) {
        movement.y += 1.0;
    }

    if movement != Vec3::ZERO {
        let movement = movement.normalize() * speed * time.delta_seconds();
        camera_transform.translation += movement;
    }
}
