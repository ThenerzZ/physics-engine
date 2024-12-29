use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            apply_gravity,
            handle_collisions,
        ));
    }
}

#[derive(Component)]
pub struct PhysicsObject {
    pub mass: f32,
}

fn apply_gravity(
    mut query: Query<(&PhysicsObject, &mut Transform)>,
    time: Res<Time>,
) {
    const GRAVITY: f32 = -9.81;
    
    for (_physics_object, mut transform) in query.iter_mut() {
        transform.translation.y += GRAVITY * time.delta_seconds();
    }
}

fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                println!("Collision started between entities: {:?} and {:?}", entity1, entity2);
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                println!("Collision stopped between entities: {:?} and {:?}", entity1, entity2);
            }
        }
    }
}
