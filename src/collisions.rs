use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct ColliderBundle {
    rigid_body: RigidBody,
    collider: Collider,
    sensor: Sensor,
    active_collision_types: ActiveCollisionTypes,
}

impl ColliderBundle {
    pub fn kinematic(collider: Collider) -> Self {
        Self {
            rigid_body: RigidBody::KinematicVelocityBased,
            collider,
            sensor: Sensor,
            active_collision_types: ActiveCollisionTypes::KINEMATIC_KINEMATIC,
        }
    }
}

pub fn intersections_with(
    collider: Entity,
    context: &RapierContext,
) -> impl Iterator<Item = Entity> + '_ {
    context
        .intersections_with(collider)
        .filter_map(move |(entity_1, entity_2, intersects)| match intersects {
            true => match entity_1 == collider {
                true => Some(entity_2),
                false => Some(entity_1),
            },
            false => None,
        })
}
