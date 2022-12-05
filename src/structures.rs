use std::time::Duration;

use bevy::{prelude::*, sprite::Anchor, time::Stopwatch};
use bevy_rapier2d::prelude::*;

use crate::{
    collisions::{intersections_with, ColliderBundle},
    palette,
    subjects::{
        states::MovingState,
        weapons::{Bow, Sword},
        Health, Speed, Subject,
    },
    Kingdom, WORLD_EXTENSION, WORLD_HEIGHT,
};

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(tick_spawners)
            .add_system(check_traps.after(tick_spawners));
    }
}

fn setup(mut commands: Commands) {
    // spawners
    commands.spawn((
        Name::new("Human spawner"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.1, 0.1, 0.5),
                custom_size: Some(Vec2::new(1.3, 1.9)),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            transform: Transform::from_xyz(-WORLD_EXTENSION + 5.0, 0.0, 0.0),
            ..default()
        },
        Kingdom::Human,
        Spawner::new("Human", palette::LIGHT_PINK, Vec2::new(1.1, 1.8), 1.5, 12.0),
    ));
    commands.spawn((
        Name::new("Monster spawner"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.1, 0.1, 0.5),
                custom_size: Some(Vec2::new(1.2, 1.5)),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            transform: Transform::from_xyz(WORLD_EXTENSION - 5.0, 0.0, 0.0),
            ..default()
        },
        Kingdom::Monster,
        Spawner::new(
            "Monster",
            palette::DARK_BLACK,
            Vec2::new(1.0, 1.4),
            2.5,
            2.0,
        ),
    ));
    // traps
    commands.spawn((
        Name::new("Human trap"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.1, 0.1, 0.1, 0.1),
                custom_size: Some(Vec2::new(10.0, WORLD_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(-WORLD_EXTENSION + 6.0, WORLD_HEIGHT / 2.0, 0.0),
            ..default()
        },
        ColliderBundle::kinematic(Collider::cuboid(5.0, WORLD_HEIGHT / 2.0)),
        Kingdom::Human,
        Trap,
    ));
    commands.spawn((
        Name::new("Monster trap"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.1, 0.1, 0.1, 0.1),
                custom_size: Some(Vec2::new(10.0, WORLD_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(WORLD_EXTENSION - 6.0, WORLD_HEIGHT / 2.0, 0.0),
            ..default()
        },
        ColliderBundle::kinematic(Collider::cuboid(5.0, WORLD_HEIGHT / 2.0)),
        Kingdom::Monster,
        Trap,
    ));
}

#[derive(Component)]
struct Spawner {
    name: String,
    color: Color,
    size: Vec2,
    speed: f32,
    stopwatch: Stopwatch,
    average_interval: Duration,
    next_interval: Duration,
}

impl Spawner {
    fn new(
        name: impl Into<String>,
        color: Color,
        size: Vec2,
        speed: f32,
        interval_seconds: f32,
    ) -> Self {
        Self {
            name: name.into(),
            color,
            size,
            speed,
            stopwatch: Stopwatch::new(),
            average_interval: Duration::from_secs_f32(interval_seconds),
            next_interval: Duration::ZERO,
        }
    }
}

#[derive(Component)]
struct Trap;

fn tick_spawners(
    mut query: Query<(&Transform, &Kingdom, &mut Spawner)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (transform, kingdom, mut spawner) in &mut query {
        spawner.stopwatch.tick(time.delta());
        while spawner.stopwatch.elapsed() >= spawner.next_interval {
            let remaining = spawner.stopwatch.elapsed() - spawner.next_interval;
            spawner.stopwatch.set_elapsed(remaining);
            let random_offset = 0.5 + fastrand::f32();
            spawner.next_interval = spawner.average_interval.mul_f32(random_offset);
            let mut spawn_commands = commands.spawn((
                Name::new(spawner.name.clone()),
                SpriteBundle {
                    sprite: Sprite {
                        color: spawner.color,
                        custom_size: Some(spawner.size),
                        ..default()
                    },
                    transform: Transform::from_translation(
                        transform.translation + Vec3::new(0.0, spawner.size.y / 2.0, 0.0),
                    ),
                    ..default()
                },
                ColliderBundle::kinematic(Collider::cuboid(
                    spawner.size.x / 2.0,
                    spawner.size.y / 2.0,
                )),
                Velocity::zero(),
                kingdom.clone(),
                Subject,
                Health::new(1),
                Speed::new(spawner.speed),
                MovingState,
            ));
            match kingdom {
                Kingdom::Human => {
                    spawn_commands.insert(Bow::new(3.0));
                }
                Kingdom::Monster => {
                    spawn_commands.insert(Sword);
                }
            }
        }
    }
}

fn check_traps(
    trap_query: Query<(Entity, &Kingdom), With<Trap>>,
    trigger_query: Query<&Kingdom, With<Subject>>,
    mut health_query: Query<(&Kingdom, &mut Health), With<Subject>>,
    context: Res<RapierContext>,
) {
    for (trap_entity, trap_kingdom) in &trap_query {
        for trigger_entity in intersections_with(trap_entity, &context) {
            let Ok(trigger_kingdom) = trigger_query.get(trigger_entity) else {
                continue;
            };
            if trigger_kingdom == trap_kingdom {
                continue;
            }
            for (subject_kingdom, mut health) in &mut health_query {
                if subject_kingdom == trigger_kingdom {
                    health.kill();
                }
            }
        }
    }
}
