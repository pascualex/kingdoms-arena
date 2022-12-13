pub mod states;
pub mod weapons;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::Kingdom;

use self::{
    states::{MovingState, ShootingState, SubjectStatesPlugin, UpdateSubjectState},
    weapons::WeaponsPlugin,
};

pub struct SubjectsPlugin;

impl Plugin for SubjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SubjectStatesPlugin)
            .add_plugin(WeaponsPlugin)
            .init_resource::<Frontlines>()
            .add_system(animate_sprite)
            .add_system(set_subject_velocities.after(UpdateSubjectState))
            .add_system(update_frontlines)
            .add_system(despawn_dead_subjects);
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
pub struct Subject;

#[derive(Component)]
pub struct Health {
    current: u32,
}

impl Health {
    pub fn new(initial: u32) -> Self {
        Self { current: initial }
    }

    pub fn damage(&mut self, amount: u32) {
        self.current = self.current.saturating_sub(amount);
    }

    pub fn kill(&mut self) {
        self.current = 0;
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Speed(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct Visuals(pub Entity);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub length: usize,
}

impl AnimationIndices {
    pub fn new(first: usize, length: usize) -> Self {
        Self { first, length }
    }

    pub fn set(&mut self, first: usize, length: usize) {
        self.first = first;
        self.length = length;
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    pub fn new(interval_seconds: f32) -> Self {
        Self(Timer::from_seconds(interval_seconds, TimerMode::Repeating))
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        let index = sprite.index - indices.first;
        let new_index = (index + timer.times_finished_this_tick() as usize) % indices.length;
        sprite.index = new_index + indices.first;
    }
}

pub fn despawn_dead_subjects(
    query: Query<(Entity, &Health), With<Subject>>,
    mut commands: Commands,
) {
    for (entity, health) in &query {
        if health.is_dead() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn set_subject_velocities(
    mut query: Query<(&mut Velocity, &Kingdom, &Speed, Option<&MovingState>), With<Subject>>,
) {
    for (mut velocity, kingdom, speed, moving_state) in &mut query {
        velocity.linvel.x = match moving_state {
            Some(_) => match kingdom {
                Kingdom::Human => **speed,
                Kingdom::Monster => -**speed,
            },
            None => 0.0,
        };
    }
}

fn update_frontlines(
    query: Query<(Entity, &Transform, &Kingdom), With<Subject>>,
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
