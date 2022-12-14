use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct ColliderBundle {
    collider: Collider,
    sensor: Sensor,
    active_collision_types: ActiveCollisionTypes,
}

impl ColliderBundle {
    pub fn new(collider: Collider) -> Self {
        Self {
            collider,
            sensor: Sensor,
            active_collision_types: ActiveCollisionTypes::KINEMATIC_KINEMATIC
                | ActiveCollisionTypes::KINEMATIC_STATIC,
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
