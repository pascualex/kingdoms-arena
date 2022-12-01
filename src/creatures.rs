use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierContext;

use crate::{collisions::intersections_with, Kingdom};

pub struct CreaturesPlugin;

impl Plugin for CreaturesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Frontlines>()
            .add_system(movement)
            .add_system(attack)
            .add_system(frontlines);
    }
}

#[derive(Resource, Default)]
struct Frontlines {
    human: Option<Entity>,
    monster: Option<Entity>,
}

#[derive(Component)]
pub struct Creature;

#[derive(Component)]
pub struct Speed {
    value: f32,
}

impl Speed {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

fn movement(mut query: Query<(&mut Transform, &Kingdom, &Speed)>, time: Res<Time>) {
    for (mut transform, kingdom, speed) in &mut query {
        transform.translation.x += match kingdom {
            Kingdom::Human => time.delta_seconds() * speed.value,
            Kingdom::Monster => -time.delta_seconds() * speed.value,
        };
    }
}

fn attack(
    attacker_query: Query<(Entity, &Kingdom), With<Creature>>,
    attacked_query: Query<&Kingdom, With<Creature>>,
    context: Res<RapierContext>,
    mut commands: Commands,
) {
    for (attacker_entity, attacker_kingdom) in &attacker_query {
        for attacked_entity in intersections_with(attacker_entity, &context) {
            let attacked_kingdom = match attacked_query.get(attacked_entity) {
                Ok(kingdom) => kingdom,
                Err(_) => continue,
            };
            if attacked_kingdom != attacker_kingdom {
                commands.entity(attacked_entity).despawn_recursive();
            }
        }
    }
}

fn frontlines(
    query: Query<(Entity, &Transform, &Kingdom), With<Creature>>,
    mut frontlines: ResMut<Frontlines>,
) {
    frontlines.human = None;
    frontlines.monster = None;

    let mut human_position = f32::NEG_INFINITY;
    let mut monster_position = f32::INFINITY;

    for (entity, transform, kingdom) in &query {
        match kingdom {
            Kingdom::Human => {
                if transform.translation.x > human_position {
                    frontlines.human = Some(entity);
                    human_position = transform.translation.x;
                }
            }
            Kingdom::Monster => {
                if transform.translation.x < monster_position {
                    frontlines.monster = Some(entity);
                    monster_position = transform.translation.x;
                }
            }
        }
    }
}
