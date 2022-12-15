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
            &Bow,
        ),
        (With<Subject>, With<ShootingState>),
    >,
    frontlines: Res<Frontlines>,
    mut events: EventWriter<ShotEvent>,
    mut commands: Commands,
) {
    for (entity, transform, mut player, kingdom, animations, bow) in &mut subject_query {
        if player.is_finished() || !frontline_in_range(transform, kingdom, bow, &frontlines) {
            if player.is_finished() {
                events.send(ShotEvent::new(entity));
            }
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
            &Bow,
        ),
        (With<Subject>, With<MovingState>),
    >,
    frontlines: Res<Frontlines>,
    mut commands: Commands,
) {
    for (entity, transform, mut player, kingdom, animations, bow) in &mut subject_query {
        if frontline_in_range(transform, kingdom, bow, &frontlines) {
            player.set(&animations.shooting, AnimationMode::Once);
            commands
                .entity(entity)
                .insert(ShootingState)
                .remove::<MovingState>();
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
