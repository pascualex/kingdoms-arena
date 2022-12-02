use bevy::prelude::*;

use crate::{creatures::Frontlines, Kingdom};

pub struct CreaturesStatePlugin;

impl Plugin for CreaturesStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(transition_to_advancing)
            .add_system(transition_to_shooting);
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct AdvancingState;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ShootingState;

fn transition_to_advancing(
    query: Query<(Entity, &Transform, &Kingdom), With<ShootingState>>,
    frontlines: Res<Frontlines>,
    mut commands: Commands,
) {
    for (entity, transform, kingdom) in &query {
        if !near_enemy_frontline(transform, kingdom, &frontlines) {
            commands
                .entity(entity)
                .insert(AdvancingState)
                .remove::<ShootingState>();
        }
    }
}

fn transition_to_shooting(
    query: Query<(Entity, &Transform, &Kingdom), With<AdvancingState>>,
    frontlines: Res<Frontlines>,
    mut commands: Commands,
) {
    for (entity, transform, kingdom) in &query {
        if near_enemy_frontline(transform, kingdom, &frontlines) {
            commands
                .entity(entity)
                .insert(ShootingState)
                .remove::<AdvancingState>();
        }
    }
}

fn near_enemy_frontline(transform: &Transform, kingdom: &Kingdom, frontlines: &Frontlines) -> bool {
    match kingdom {
        Kingdom::Human => (frontlines.monster.position - transform.translation.x) < 5.0,
        Kingdom::Monster => false,
    }
}
