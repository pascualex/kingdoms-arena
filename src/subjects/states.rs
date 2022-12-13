use bevy::prelude::*;

use crate::{
    subjects::{AnimationIndices, Frontlines, Subject, Visuals},
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
pub struct ShootingState;

fn transition_to_moving(
    subject_query: Query<
        (Entity, &Transform, &Visuals, &Kingdom),
        (With<Subject>, With<ShootingState>),
    >,
    mut visuals_query: Query<(&mut TextureAtlasSprite, &mut AnimationIndices)>,
    frontlines: Res<Frontlines>,
    mut commands: Commands,
) {
    for (entity, transform, visuals, kingdom) in &subject_query {
        if !near_enemy_frontline(transform, kingdom, &frontlines) {
            if let Ok((mut sprite, mut indices)) = visuals_query.get_mut(**visuals) {
                indices.set(9, 4);
                sprite.index = indices.first;
            }
            commands
                .entity(entity)
                .insert(MovingState)
                .remove::<ShootingState>();
        }
    }
}

fn transition_to_shooting(
    subject_query: Query<
        (Entity, &Transform, &Visuals, &Kingdom),
        (With<Subject>, With<MovingState>),
    >,
    mut visuals_query: Query<(&mut TextureAtlasSprite, &mut AnimationIndices)>,
    frontlines: Res<Frontlines>,
    mut commands: Commands,
) {
    for (entity, transform, visuals, kingdom) in &subject_query {
        if near_enemy_frontline(transform, kingdom, &frontlines) {
            if let Ok((mut sprite, mut indices)) = visuals_query.get_mut(**visuals) {
                indices.set(0, 2);
                sprite.index = indices.first;
            }
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
