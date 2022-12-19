use bevy::prelude::*;

use crate::{
    animation::{AnimationMode, AnimationPlayer},
    subjects::{Frontlines, Subject, SubjectAnimations},
    weapons::{Bow, ShotEvent},
    Kingdom,
};

#[derive(SystemLabel)]
pub struct UpdateSubjectState;

pub struct SubjectStatePlugin;

impl Plugin for SubjectStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_moving_transitions.label(UpdateSubjectState))
            .add_system(check_recharging_transitions.label(UpdateSubjectState))
            .add_system(check_shooting_transitions.label(UpdateSubjectState));
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct MovingState;

impl MovingState {
    fn transition(player: &mut AnimationPlayer, animations: &SubjectAnimations) {
        player.set(&animations.moving, AnimationMode::Repeating);
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct RechargingState;

impl RechargingState {
    fn transition(player: &mut AnimationPlayer, animations: &SubjectAnimations) {
        player.set(&animations.idle, AnimationMode::Repeating);
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ShootingState;

impl ShootingState {
    fn transition(player: &mut AnimationPlayer, animations: &SubjectAnimations) {
        player.set(&animations.shooting, AnimationMode::Once);
    }
}

fn check_moving_transitions(
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
                ShootingState::transition(&mut player, animations);
                commands.entity(entity).insert(ShootingState);
            } else {
                RechargingState::transition(&mut player, animations);
                commands.entity(entity).insert(RechargingState);
            }
        }
    }
}

fn check_recharging_transitions(
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
            MovingState::transition(&mut player, animations);
            commands.entity(entity).insert(MovingState);
        } else if bow.timer.finished() {
            commands.entity(entity).remove::<RechargingState>();
            ShootingState::transition(&mut player, animations);
            commands.entity(entity).insert(ShootingState);
        }
    }
}

fn check_shooting_transitions(
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
            MovingState::transition(&mut player, animations);
            commands.entity(entity).insert(MovingState);
        } else if player.is_finished() {
            commands.entity(entity).remove::<ShootingState>();
            RechargingState::transition(&mut player, animations);
            commands.entity(entity).insert(RechargingState);
            events.send(ShotEvent::new(entity));
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
        Kingdom::Elven => (frontlines.monster.position - transform.translation.x) < bow.range,
        Kingdom::Monster => (transform.translation.x - frontlines.elven.position) < bow.range,
    }
}
