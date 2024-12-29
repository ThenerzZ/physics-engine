use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_rapier3d::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
           .init_resource::<UiState>()
           .add_systems(Update, (
               ui_system,
               toolbar_system,
               handle_selection.after(ui_system),  // Run after UI to check if UI was clicked
               handle_transform_tools,
           ));
    }
}

#[derive(Resource, Default)]
struct UiState {
    selected_entity: Option<Entity>,
    selected_tool: Tool,
    selected_shape: ShapeType,
    dragging: bool,
    drag_start: Option<Vec2>,
    ui_received_click: bool,
}

#[derive(Default, PartialEq, Clone, Copy)]
enum Tool {
    #[default]
    Select,
    Move,
    Rotate,
    Scale,
}

#[derive(Default, PartialEq, Clone, Copy)]
enum ShapeType {
    #[default]
    Cube,
    Sphere,
    Cylinder,
    Cone,
    Capsule,
}

// Component to mark selectable objects
#[derive(Component)]
struct Selectable;

fn spawn_shape(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    shape_type: ShapeType,
    position: Vec3,
) -> Entity {
    let (mesh, collider) = match shape_type {
        ShapeType::Cube => (
            Mesh::from(shape::Box::new(1.0, 1.0, 1.0)),
            Collider::cuboid(0.5, 0.5, 0.5),
        ),
        ShapeType::Sphere => (
            Mesh::from(shape::UVSphere {
                radius: 0.5,
                sectors: 32,
                stacks: 16,
            }),
            Collider::ball(0.5),
        ),
        ShapeType::Cylinder => (
            Mesh::from(shape::Cylinder {
                radius: 0.5,
                height: 1.0,
                resolution: 32,
                segments: 1,
            }),
            Collider::cylinder(0.5, 0.5),
        ),
        ShapeType::Cone => {
            // Create a cone by scaling a cylinder
            let mut cone_mesh = Mesh::from(shape::Cylinder {
                radius: 0.5,
                height: 1.0,
                resolution: 32,
                segments: 1,
            });
            
            // Modify the vertices to create a cone shape
            if let Some(positions) = cone_mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                if let bevy::render::mesh::VertexAttributeValues::Float32x3(positions) = positions {
                    for position in positions.iter_mut() {
                        if position[1] > 0.0 { // Top vertices
                            position[0] = 0.0;
                            position[2] = 0.0;
                        }
                    }
                }
            }
            
            (
                cone_mesh,
                // For cone we'll use a custom compound collider
                Collider::compound(vec![
                    // Cone base (cylinder with small height)
                    (
                        Vec3::new(0.0, -0.45, 0.0),
                        Quat::IDENTITY,
                        Collider::cylinder(0.5, 0.1),
                    ),
                    // Cone body (approximated with cylinder)
                    (
                        Vec3::ZERO,
                        Quat::IDENTITY,
                        Collider::cylinder(0.25, 0.8),
                    ),
                ]),
            )
        },
        ShapeType::Capsule => (
            Mesh::from(shape::Capsule {
                radius: 0.5,
                rings: 16,
                depth: 1.0,
                latitudes: 16,
                longitudes: 32,
                uv_profile: default(),
            }),
            Collider::capsule_y(0.5, 0.5), // height/2, radius
        ),
    };

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(position),
            ..default()
        },
        Selectable,
        RigidBody::Dynamic,
        collider,
        ColliderMassProperties::Mass(1.0),
        Restitution::coefficient(0.7),
        Friction::coefficient(0.5),
        Damping {
            linear_damping: 0.5,
            angular_damping: 0.5,
        },
    )).id()
}

fn handle_selection(
    mut ui_state: ResMut<UiState>,
    mouse_button: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
    mut contexts: EguiContexts,
) {
    // Only handle selection if UI didn't receive the click and we're not hovering over UI
    if !ui_state.ui_received_click && 
       !contexts.ctx_mut().is_pointer_over_area() &&
       ui_state.selected_tool == Tool::Select && 
       mouse_button.just_pressed(MouseButton::Left) 
    {
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok((camera, camera_transform)) = cameras.get_single() {
                    if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                        let ray_pos = ray.origin;
                        let ray_dir = ray.direction;

                        if let Some((entity, _toi)) = rapier_context.cast_ray(
                            ray_pos,
                            ray_dir,
                            f32::MAX,
                            true,
                            QueryFilter::default(),
                        ) {
                            ui_state.selected_entity = Some(entity);
                        } else {
                            // Only deselect if we're not over any UI element
                            ui_state.selected_entity = None;
                        }
                    }
                }
            }
        }
    }
}

fn handle_transform_tools(
    mut ui_state: ResMut<UiState>,
    mouse_button: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    mut transforms: Query<&mut Transform>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut contexts: EguiContexts,
) {
    // Don't handle transform tools if we're over UI
    if contexts.ctx_mut().is_pointer_over_area() {
        return;
    }

    if ui_state.selected_tool != Tool::Select {
        if let Some(selected_entity) = ui_state.selected_entity {
            if let Ok(window) = windows.get_single() {
                if let Some(cursor_pos) = window.cursor_position() {
                    if mouse_button.just_pressed(MouseButton::Left) {
                        ui_state.dragging = true;
                        ui_state.drag_start = Some(cursor_pos);
                    } else if mouse_button.just_released(MouseButton::Left) {
                        ui_state.dragging = false;
                        ui_state.drag_start = None;
                    }

                    if ui_state.dragging {
                        if let Some(drag_start) = ui_state.drag_start {
                            let delta = cursor_pos - drag_start;
                            if let Ok(mut transform) = transforms.get_mut(selected_entity) {
                                match ui_state.selected_tool {
                                    Tool::Move => {
                                        if let Ok((camera, camera_transform)) = cameras.get_single() {
                                            let forward = -camera_transform.forward();
                                            let right = camera_transform.right();
                                            let movement = right * delta.x * 0.01 + forward * delta.y * 0.01;
                                            transform.translation += movement;
                                        }
                                    }
                                    Tool::Rotate => {
                                        let rotation = Quat::from_euler(
                                            EulerRot::XYZ,
                                            delta.y * 0.01,
                                            delta.x * 0.01,
                                            0.0,
                                        );
                                        transform.rotate(rotation);
                                    }
                                    Tool::Scale => {
                                        let scale_factor = 1.0 + delta.x * 0.01;
                                        transform.scale *= scale_factor;
                                    }
                                    _ => {}
                                }
                            }
                            ui_state.drag_start = Some(cursor_pos);
                        }
                    }
                }
            }
        }
    }
}

fn toolbar_system(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    egui::TopBottomPanel::top("toolbar").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            // Tools section
            ui.group(|ui| {
                ui.label("Tools:");
                ui.selectable_value(&mut ui_state.selected_tool, Tool::Select, "🖱️ Select");
                ui.selectable_value(&mut ui_state.selected_tool, Tool::Move, "↔️ Move");
                ui.selectable_value(&mut ui_state.selected_tool, Tool::Rotate, "🔄 Rotate");
                ui.selectable_value(&mut ui_state.selected_tool, Tool::Scale, "⇲ Scale");
            });
            
            ui.separator();
            
            // Add objects dropdown
            ui.group(|ui| {
                ui.label("Add Object:");
                egui::ComboBox::from_id_source("add_object")
                    .selected_text("➕ Add Object")
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(false, "📦 Cube").clicked() {
                            spawn_shape(&mut commands, &mut meshes, &mut materials, ShapeType::Cube, Vec3::new(0.0, 2.0, 0.0));
                        }
                        if ui.selectable_label(false, "⚪ Sphere").clicked() {
                            spawn_shape(&mut commands, &mut meshes, &mut materials, ShapeType::Sphere, Vec3::new(0.0, 2.0, 0.0));
                        }
                        if ui.selectable_label(false, "🛢️ Cylinder").clicked() {
                            spawn_shape(&mut commands, &mut meshes, &mut materials, ShapeType::Cylinder, Vec3::new(0.0, 2.0, 0.0));
                        }
                        if ui.selectable_label(false, "🔺 Cone").clicked() {
                            spawn_shape(&mut commands, &mut meshes, &mut materials, ShapeType::Cone, Vec3::new(0.0, 2.0, 0.0));
                        }
                        if ui.selectable_label(false, "💊 Capsule").clicked() {
                            spawn_shape(&mut commands, &mut meshes, &mut materials, ShapeType::Capsule, Vec3::new(0.0, 2.0, 0.0));
                        }
                    });
            });
        });
    });
}

fn ui_system(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut RigidBody,
        Option<&Collider>,
        Option<&ColliderMassProperties>,
        &mut Damping,
        &mut Restitution,
        &mut Friction,
    )>,
) {
    // Reset UI click state at the start of each frame
    ui_state.ui_received_click = contexts.ctx_mut().is_pointer_over_area();

    egui::Window::new("Inspector")
        .default_width(280.0)
        .default_height(600.0)
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            // Set UI click state if the window is interacted with
            if ui.ui_contains_pointer() && ui.input(|i| i.pointer.any_click()) {
                ui_state.ui_received_click = true;
            }

            match ui_state.selected_entity {
                Some(selected_entity) => {
                    if let Ok((entity, mut transform, mut rigid_body, _collider, _mass, mut damping, mut restitution, mut friction)) 
                        = query.get_mut(selected_entity) 
                    {
                        ui.label(format!("Entity {:?}", entity));

                        // Transform section
                        ui.collapsing("Transform", |ui| {
                            let mut position = transform.translation.to_array();
                            let mut rotation = transform.rotation.to_euler(EulerRot::XYZ);
                            let mut scale = transform.scale.to_array();

                            ui.group(|ui| {
                                ui.label("Position");
                                if ui.add(egui::DragValue::new(&mut position[0]).prefix("X: ").speed(0.1)).changed() {
                                    transform.translation.x = position[0];
                                }
                                if ui.add(egui::DragValue::new(&mut position[1]).prefix("Y: ").speed(0.1)).changed() {
                                    transform.translation.y = position[1];
                                }
                                if ui.add(egui::DragValue::new(&mut position[2]).prefix("Z: ").speed(0.1)).changed() {
                                    transform.translation.z = position[2];
                                }
                            });

                            ui.group(|ui| {
                                ui.label("Rotation (radians)");
                                if ui.add(egui::DragValue::new(&mut rotation.0).prefix("X: ").speed(0.01)).changed() {
                                    transform.rotation = Quat::from_euler(EulerRot::XYZ, rotation.0, rotation.1, rotation.2);
                                }
                                if ui.add(egui::DragValue::new(&mut rotation.1).prefix("Y: ").speed(0.01)).changed() {
                                    transform.rotation = Quat::from_euler(EulerRot::XYZ, rotation.0, rotation.1, rotation.2);
                                }
                                if ui.add(egui::DragValue::new(&mut rotation.2).prefix("Z: ").speed(0.01)).changed() {
                                    transform.rotation = Quat::from_euler(EulerRot::XYZ, rotation.0, rotation.1, rotation.2);
                                }
                            });

                            ui.group(|ui| {
                                ui.label("Scale");
                                if ui.add(egui::DragValue::new(&mut scale[0]).prefix("X: ").speed(0.1)).changed() {
                                    transform.scale.x = scale[0];
                                }
                                if ui.add(egui::DragValue::new(&mut scale[1]).prefix("Y: ").speed(0.1)).changed() {
                                    transform.scale.y = scale[1];
                                }
                                if ui.add(egui::DragValue::new(&mut scale[2]).prefix("Z: ").speed(0.1)).changed() {
                                    transform.scale.z = scale[2];
                                }

                                if ui.button("Make Uniform").clicked() {
                                    transform.scale = Vec3::splat(transform.scale.x);
                                }
                            });
                        });

                        // Physics section
                        ui.collapsing("Physics", |ui| {
                            ui.group(|ui| {
                                ui.label("Body Type");
                                ui.horizontal(|ui| {
                                    if ui.selectable_label(*rigid_body == RigidBody::Dynamic, "Dynamic").clicked() {
                                        *rigid_body = RigidBody::Dynamic;
                                    }
                                    if ui.selectable_label(*rigid_body == RigidBody::Fixed, "Fixed").clicked() {
                                        *rigid_body = RigidBody::Fixed;
                                    }
                                    if ui.selectable_label(*rigid_body == RigidBody::KinematicPositionBased, "Kinematic").clicked() {
                                        *rigid_body = RigidBody::KinematicPositionBased;
                                    }
                                });
                            });

                            ui.group(|ui| {
                                ui.label("Damping");
                                let mut linear = damping.linear_damping;
                                let mut angular = damping.angular_damping;
                                if ui.add(egui::Slider::new(&mut linear, 0.0..=1.0).text("Linear")).changed() {
                                    damping.linear_damping = linear;
                                }
                                if ui.add(egui::Slider::new(&mut angular, 0.0..=1.0).text("Angular")).changed() {
                                    damping.angular_damping = angular;
                                }
                            });

                            ui.group(|ui| {
                                ui.label("Restitution (Bounciness)");
                                let mut value = restitution.coefficient;
                                if ui.add(egui::Slider::new(&mut value, 0.0..=1.0)).changed() {
                                    restitution.coefficient = value;
                                }
                            });

                            ui.group(|ui| {
                                ui.label("Friction");
                                let mut value = friction.coefficient;
                                if ui.add(egui::Slider::new(&mut value, 0.0..=1.0)).changed() {
                                    friction.coefficient = value;
                                }
                            });
                        });

                        ui.separator();
                        ui.horizontal(|ui| {
                            if ui.button("Add Physics").clicked() {
                                // TODO: Add physics components
                            }
                            if ui.button("Remove Physics").clicked() {
                                // TODO: Remove physics components
                            }
                        });
                    } else {
                        ui.label("Selected entity no longer exists");
                    }
                }
                None => {
                    ui.label("No entity selected");
                    ui.label("Click an object to select it");
                }
            }
        });
}
