use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::{Anchor, TextureAtlas},
    time::Stopwatch,
};
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    collision::{intersections_with, ColliderBundle},
    subjects::{
        content::{SubjectBlueprint, ELVEN_ARCHER, GOBLIN_WARRIOR},
        spawn_subject, Health, Subject,
    },
    Kingdom, SKY_HEIGHT, WORLD_EXTENSION,
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
        Spawner::new(ELVEN_ARCHER, 12.0),
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
        Spawner::new(GOBLIN_WARRIOR, 2.0),
    ));
    // traps
    commands.spawn((
        Name::new("Human trap"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.1, 0.1, 0.1, 0.1),
                custom_size: Some(Vec2::new(10.0, SKY_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(-WORLD_EXTENSION + 6.0, SKY_HEIGHT / 2.0, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        ColliderBundle::new(Collider::cuboid(5.0, SKY_HEIGHT / 2.0)),
        Kingdom::Human,
        Trap,
    ));
    commands.spawn((
        Name::new("Monster trap"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.1, 0.1, 0.1, 0.1),
                custom_size: Some(Vec2::new(10.0, SKY_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(WORLD_EXTENSION - 6.0, SKY_HEIGHT / 2.0, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        ColliderBundle::new(Collider::cuboid(5.0, SKY_HEIGHT / 2.0)),
        Kingdom::Monster,
        Trap,
    ));
}

#[derive(Component)]
struct Spawner {
    subject_blueprint: SubjectBlueprint,
    stopwatch: Stopwatch,
    average_interval: Duration,
    next_interval: Duration,
}

impl Spawner {
    fn new(subject_blueprint: SubjectBlueprint, interval_seconds: f32) -> Self {
        Self {
            subject_blueprint,
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
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    for (transform, kingdom, mut spawner) in &mut query {
        spawner.stopwatch.tick(time.delta());

        while spawner.stopwatch.elapsed() >= spawner.next_interval {
            let remaining = spawner.stopwatch.elapsed() - spawner.next_interval;
            spawner.stopwatch.set_elapsed(remaining);

            let random_offset = 0.5 + fastrand::f32();
            spawner.next_interval = spawner.average_interval.mul_f32(random_offset);

            spawn_subject(
                &spawner.subject_blueprint,
                transform.translation,
                *kingdom,
                &asset_server,
                &mut texture_atlases,
                &mut commands,
            );

            let sound = match kingdom {
                Kingdom::Human => asset_server.load("sounds/human_spawn.wav"),
                Kingdom::Monster => asset_server.load("sounds/monster_spawn.wav"),
            };
            audio.play(sound).with_volume(0.1);
        }
    }
}

fn check_traps(
    trap_query: Query<(Entity, &Kingdom), With<Trap>>,
    trigger_query: Query<&Kingdom, With<Subject>>,
    mut health_query: Query<(&Kingdom, &mut Health), With<Subject>>,
    context: Res<RapierContext>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
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

            let sound = asset_server.load("sounds/wipe_out.wav");
            audio.play(sound).with_volume(0.5);
        }
    }
}
