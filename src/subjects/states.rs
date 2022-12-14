use bevy::prelude::*;

use crate::{
    animation::{AnimationMode, AnimationPlayer},
    subjects::{Frontlines, Subject, SubjectAnimations},
    Kingdom,
};

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
pub struct RecharginState;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ShootingState;

fn transition_to_moving(
    mut subject_query: Query<
        (
            Entity,
            &Transform,
            &mut AnimationPlayer,
            &Kingdom,
            &SubjectAnimations,
        ),
        (With<Subject>, With<ShootingState>),
    >,
    frontlines: Res<Frontlines>,
    mut commands: Commands,
) {
    for (entity, transform, mut player, kingdom, animations) in &mut subject_query {
        if !near_enemy_frontline(transform, kingdom, &frontlines) {
            player.set(&animations.moving, AnimationMode::Repeating);
            commands
                .entity(entity)
                .insert(MovingState)
                .remove::<ShootingState>();
        }
    }
}

fn transition_to_shooting(
    mut subject_query: Query<
        (
            Entity,
            &Transform,
            &mut AnimationPlayer,
            &Kingdom,
            &SubjectAnimations,
        ),
        (With<Subject>, With<MovingState>),
    >,
    frontlines: Res<Frontlines>,
    mut commands: Commands,
) {
    for (entity, transform, mut player, kingdom, animations) in &mut subject_query {
        if near_enemy_frontline(transform, kingdom, &frontlines) {
            player.set(&animations.shooting, AnimationMode::Repeating);
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
