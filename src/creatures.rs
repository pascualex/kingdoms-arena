use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierContext;

use crate::{collisions::intersections_with, Kingdom};

pub struct CreaturesPlugin;

impl Plugin for CreaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement).add_system(attack);
    }
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

#[derive(Component, Clone)]
pub enum Behaviour {
    MoveRight,
    MoveLeft,
}

fn movement(mut query: Query<(&mut Transform, &Speed, &Behaviour)>, time: Res<Time>) {
    for (mut transform, speed, behaviour) in &mut query {
        transform.translation.x += match behaviour {
            Behaviour::MoveRight => time.delta_seconds() * speed.value,
            Behaviour::MoveLeft => -time.delta_seconds() * speed.value,
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
