pub mod states;
pub mod weapons;

use bevy::prelude::*;

use crate::Kingdom;

use self::{
    states::{MovingState, ShootingState, SubjectStatesPlugin},
    weapons::WeaponsPlugin,
};

pub struct SubjectsPlugin;

impl Plugin for SubjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SubjectStatesPlugin)
            .add_plugin(WeaponsPlugin)
            .init_resource::<Frontlines>()
            .add_system(move_subjects)
            .add_system(update_frontlines.after(move_subjects))
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

#[derive(Component)]
pub struct Speed {
    value: f32,
}

impl Speed {
    pub fn new(value: f32) -> Self {
        Self { value }
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

pub fn move_subjects(
    mut query: Query<(&mut Transform, &Kingdom, &Speed), (With<Subject>, With<MovingState>)>,
    time: Res<Time>,
) {
    for (mut transform, kingdom, speed) in &mut query {
        transform.translation.x += match kingdom {
            Kingdom::Human => time.delta_seconds() * speed.value,
            Kingdom::Monster => -time.delta_seconds() * speed.value,
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
