use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;

use crate::{
    collisions::{intersections_with, ColliderBundle},
    creatures::{AdvancingState, Creature, Speed},
    palette, Kingdom, WORLD_EXTENSION, WORLD_HEIGHT,
};

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(tick_spawners)
            .add_system(check_traps);
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
        Spawner::new("Human", palette::LIGHT_PINK, Vec2::new(1.1, 1.8), 1.0, 2.0),
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
            2.0,
            1.0,
        ),
    ));
    // wipeout traps
    commands.spawn((
        Name::new("Human wipe out trap"),
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
        Name::new("Monster wipe out trap"),
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
    timer: Timer,
    spawn_count: u32,
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
            timer: Timer::from_seconds(interval_seconds, TimerMode::Repeating),
            spawn_count: 0,
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
        spawner.timer.tick(time.delta());
        for _ in 0..spawner.timer.times_finished_this_tick() {
            spawner.spawn_count += 1;
            commands.spawn((
                Name::new(format!("{} {}", spawner.name, spawner.spawn_count)),
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
                kingdom.clone(),
                Creature,
                Speed::new(spawner.speed),
                AdvancingState,
            ));
        }
    }
}

fn check_traps(
    trap_query: Query<(Entity, &Kingdom), With<Trap>>,
    trigger_query: Query<&Kingdom, With<Creature>>,
    creature_query: Query<(Entity, &Kingdom), With<Creature>>,
    context: Res<RapierContext>,
    mut commands: Commands,
) {
    for (trap_entity, trap_kingdom) in &trap_query {
        for trigger_entity in intersections_with(trap_entity, &context) {
            let trigger_kingdom = match trigger_query.get(trigger_entity) {
                Ok(kingdom) => kingdom,
                Err(_) => continue,
            };
            if trigger_kingdom == trap_kingdom {
                continue;
            }
            for (creature_entity, creature_kingdom) in &creature_query {
                if creature_kingdom == trigger_kingdom {
                    commands.entity(creature_entity).despawn_recursive();
                }
            }
        }
    }
}
