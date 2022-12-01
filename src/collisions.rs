use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision);
    }
}

#[derive(Bundle)]
pub struct ColliderBundle {
    rigid_body: RigidBody,
    collider: Collider,
    sensor: Sensor,
    active_collision_types: ActiveCollisionTypes,
    active_events: ActiveEvents,
}

impl ColliderBundle {
    pub fn kinematic(collider: Collider) -> Self {
        Self {
            rigid_body: RigidBody::KinematicPositionBased,
            collider,
            sensor: Sensor,
            active_collision_types: ActiveCollisionTypes::KINEMATIC_KINEMATIC,
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

fn collision(mut events: EventReader<CollisionEvent>, mut commands: Commands) {
    for event in events.iter() {
        let (entity_1, entity_2) = match event {
            CollisionEvent::Started(entity_1, entity_2, _) => (*entity_1, *entity_2),
            _ => continue,
        };
        commands.entity(entity_1).despawn_recursive();
        commands.entity(entity_2).despawn_recursive();
    }
}
