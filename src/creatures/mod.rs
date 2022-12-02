pub mod states;

use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierContext;

use crate::{collisions::intersections_with, Kingdom};

use self::states::{AdvancingState, CreaturesStatePlugin};

pub struct CreaturesPlugin;

impl Plugin for CreaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CreaturesStatePlugin)
            .init_resource::<Frontlines>()
            .add_system(advance_creatures)
            .add_system(perform_creature_attacks)
            .add_system(update_frontlines);
    }
}

#[derive(Resource)]
struct Frontlines {
    human: Frontline,
    monster: Frontline,
}

impl Default for Frontlines {
    fn default() -> Self {
        Self {
            human: Frontline {
                position: f32::NEG_INFINITY,
                entity: None,
            },
            monster: Frontline {
                position: f32::INFINITY,
                entity: None,
            },
        }
    }
}

struct Frontline {
    position: f32,
    entity: Option<Entity>,
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

#[allow(clippy::type_complexity)]
fn advance_creatures(
    mut query: Query<(&mut Transform, &Kingdom, &Speed), (With<Creature>, With<AdvancingState>)>,
    time: Res<Time>,
) {
    for (mut transform, kingdom, speed) in &mut query {
        transform.translation.x += match kingdom {
            Kingdom::Human => time.delta_seconds() * speed.value,
            Kingdom::Monster => -time.delta_seconds() * speed.value,
        };
    }
}

fn perform_creature_attacks(
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

fn update_frontlines(
    query: Query<(Entity, &Transform, &Kingdom), With<Creature>>,
    mut frontlines: ResMut<Frontlines>,
) {
    frontlines.human.position = f32::NEG_INFINITY;
    frontlines.monster.position = f32::INFINITY;

    frontlines.human.entity = None;
    frontlines.monster.entity = None;

    for (entity, transform, kingdom) in &query {
        match kingdom {
            Kingdom::Human => {
                if transform.translation.x > frontlines.human.position {
                    frontlines.human.position = transform.translation.x;
                    frontlines.human.entity = Some(entity);
                }
            }
            Kingdom::Monster => {
                if transform.translation.x < frontlines.monster.position {
                    frontlines.monster.position = transform.translation.x;
                    frontlines.monster.entity = Some(entity);
                }
            }
        }
    }
}
