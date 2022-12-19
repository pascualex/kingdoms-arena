use bevy::prelude::*;

use crate::{
    animation::{AnimationMode, AnimationPlayer},
    subjects::{Frontlines, Subject, SubjectAnimations},
    weapons::{Bow, ShotEvent},
    Kingdom,
};

#[derive(SystemLabel)]
pub struct UpdateSubjectState;

pub struct SubjectStatesPlugin;

impl Plugin for SubjectStatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(transition_from_moving.label(UpdateSubjectState))
            .add_system(transition_from_recharging.label(UpdateSubjectState))
            .add_system(transition_from_shooting.label(UpdateSubjectState));
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct MovingState;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct RechargingState;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ShootingState;

fn transition_from_moving(
    mut subject_query: Query<
        (
            Entity,
            &Transform,
            &mut AnimationPlayer,
            &Kingdom,
            &SubjectAnimations,
            &Bow,
        ),
        (With<Subject>, With<MovingState>),
    >,
    frontlines: Res<Frontlines>,
    mut commands: Commands,
) {
    for (entity, transform, mut player, kingdom, animations, bow) in &mut subject_query {
        if frontline_in_range(transform, kingdom, bow, &frontlines) {
            commands.entity(entity).remove::<MovingState>();
            if bow.timer.finished() {
                player.set(&animations.shooting, AnimationMode::Once);
                commands.entity(entity).insert(ShootingState);
            } else {
                player.set(&animations.idle, AnimationMode::Repeating);
                commands.entity(entity).insert(RechargingState);
            }
        }
    }
}

fn transition_from_recharging(
    mut subject_query: Query<
        (
            Entity,
            &Transform,
            &mut AnimationPlayer,
            &Kingdom,
            &SubjectAnimations,
            &Bow,
        ),
        (With<Subject>, With<RechargingState>),
    >,
    frontlines: Res<Frontlines>,
    mut commands: Commands,
) {
    for (entity, transform, mut player, kingdom, animations, bow) in &mut subject_query {
        if !frontline_in_range(transform, kingdom, bow, &frontlines) {
            commands.entity(entity).remove::<RechargingState>();
            player.set(&animations.moving, AnimationMode::Repeating);
            commands.entity(entity).insert(MovingState);
        } else if bow.timer.finished() {
            commands.entity(entity).remove::<RechargingState>();
            player.set(&animations.shooting, AnimationMode::Once);
            commands.entity(entity).insert(ShootingState);
        }
    }
}

fn transition_from_shooting(
    mut subject_query: Query<
        (
            Entity,
            &Transform,
            &mut AnimationPlayer,
            &Kingdom,
            &SubjectAnimations,
            &Bow,
        ),
        (With<Subject>, With<ShootingState>),
    >,
    frontlines: Res<Frontlines>,
    mut events: EventWriter<ShotEvent>,
    mut commands: Commands,
) {
    for (entity, transform, mut player, kingdom, animations, bow) in &mut subject_query {
        if !frontline_in_range(transform, kingdom, bow, &frontlines) {
            commands.entity(entity).remove::<ShootingState>();
            player.set(&animations.moving, AnimationMode::Repeating);
            commands.entity(entity).insert(MovingState);
        } else if player.is_finished() {
            commands.entity(entity).remove::<ShootingState>();
            player.set(&animations.idle, AnimationMode::Repeating);
            events.send(ShotEvent::new(entity));
            commands.entity(entity).insert(RechargingState);
        }
    }
}

fn frontline_in_range(
    transform: &Transform,
    kingdom: &Kingdom,
    bow: &Bow,
    frontlines: &Frontlines,
) -> bool {
    match kingdom {
        Kingdom::Human => (frontlines.monster.position - transform.translation.x) < bow.range,
        Kingdom::Monster => (transform.translation.x - frontlines.human.position) < bow.range,
    }
}
