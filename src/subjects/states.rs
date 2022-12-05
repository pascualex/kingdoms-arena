use bevy::prelude::*;

use crate::{subjects::Frontlines, Kingdom};

#[derive(SystemLabel)]
pub struct UpdateSubjectState;

pub struct SubjectStatesPlugin;

impl Plugin for SubjectStatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(transition_to_moving.label(UpdateSubjectState))
            .add_system(transition_to_shooting.label(UpdateSubjectState));
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct MovingState;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ShootingState;

fn transition_to_moving(
    query: Query<(Entity, &Transform, &Kingdom), With<ShootingState>>,
    frontlines: Res<Frontlines>,
    mut commands: Commands,
) {
    for (entity, transform, kingdom) in &query {
        if !near_enemy_frontline(transform, kingdom, &frontlines) {
            commands
                .entity(entity)
                .insert(MovingState)
                .remove::<ShootingState>();
        }
    }
}

fn transition_to_shooting(
    query: Query<(Entity, &Transform, &Kingdom), With<MovingState>>,
    frontlines: Res<Frontlines>,
    mut commands: Commands,
) {
    for (entity, transform, kingdom) in &query {
        if near_enemy_frontline(transform, kingdom, &frontlines) {
            commands
                .entity(entity)
                .insert(ShootingState)
                .remove::<MovingState>();
        }
    }
}

fn near_enemy_frontline(transform: &Transform, kingdom: &Kingdom, frontlines: &Frontlines) -> bool {
    match kingdom {
        Kingdom::Human => (frontlines.monster.position - transform.translation.x) < 10.0,
        Kingdom::Monster => false,
    }
}
