use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
    }
}

const BACKGROUND_COLOR: Color = Color::rgb(0.08, 0.08, 0.08);
const PANEL_COLOR: Color = Color::rgba(0.12, 0.12, 0.12, 0.95);
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const ACCENT_COLOR: Color = Color::rgb(1.0, 0.4, 0.0);
const ACCENT_COLOR_2: Color = Color::rgb(0.95, 0.2, 0.6);
const BORDER_COLOR: Color = Color::rgba(0.2, 0.2, 0.2, 0.5);
const INPUT_BG_COLOR: Color = Color::rgba(0.15, 0.15, 0.15, 0.95);

#[derive(Component)]
pub struct InspectorPanel;

fn setup_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(280.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
            background_color: PANEL_COLOR.into(),
            ..default()
        })
        .with_children(|parent| {
            // Title Section
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    padding: UiRect::bottom(Val::Px(16.0)),
                    border: UiRect::bottom(Val::Px(1.0)),
                    ..default()
                },
                border_color: BORDER_COLOR.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Inspector",
                    TextStyle {
                        font_size: 16.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                ));
            });

            // Properties Section
            spawn_property_section(parent, "Transform");
            spawn_property_field(parent, "Position X", "0.0");
            spawn_property_field(parent, "Position Y", "0.0");
            spawn_property_field(parent, "Position Z", "0.0");

            spawn_property_section(parent, "Physics");
            spawn_property_field(parent, "Mass", "1.0");
            spawn_property_field(parent, "Friction", "0.5");
            spawn_property_field(parent, "Restitution", "0.5");

            // Add Object Button
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(32.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::top(Val::Px(16.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                border_color: ACCENT_COLOR.into(),
                background_color: ACCENT_COLOR.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Add Object",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });
}

fn spawn_property_section(parent: &mut ChildBuilder, title: &str) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            padding: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(16.0), Val::Px(8.0)),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            title,
            TextStyle {
                font_size: 13.0,
                color: Color::rgb(0.5, 0.5, 0.5),
                ..default()
            },
        ));
    });
}

fn spawn_property_field(parent: &mut ChildBuilder, label: &str, default_value: &str) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            margin: UiRect::bottom(Val::Px(8.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        // Label
        parent.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font_size: 12.0,
                color: TEXT_COLOR,
                ..default()
            },
        ));

        // Input field
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(28.0),
                margin: UiRect::top(Val::Px(4.0)),
                padding: UiRect::horizontal(Val::Px(8.0)),
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            border_color: BORDER_COLOR.into(),
            background_color: INPUT_BG_COLOR.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                default_value,
                TextStyle {
                    font_size: 13.0,
                    color: TEXT_COLOR,
                    ..default()
                },
            ));
        });
    });
}
