use std::time::Duration;

use bevy::{prelude::*, sprite::Anchor, time::Stopwatch};
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    collision::{intersections_with, ColliderBundle},
    subjects::{
        content::{SubjectBlueprint, ELVEN_ARCHER, GOBLIN_WARRIOR},
        Health, SpawnEvent, Subject,
    },
    Kingdom, KingdomHandle, SKY_HEIGHT, WORLD_EXTENSION,
};

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnerEvent>()
            .add_startup_system(load_assets)
            .add_startup_system(setup)
            .add_system(trigger_spawners)
            .add_system(tick_spawners)
            .add_system(check_traps.after(tick_spawners));
    }
}

fn load_assets(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(StructureAssets {
        spawn_sound: KingdomHandle {
            elven: asset_server.load("sounds/elf_spawn.wav"),
            monster: asset_server.load("sounds/monster_spawn.wav"),
        },
    });
}

fn setup(mut commands: Commands) {
    // spawners
    commands.spawn((
        Name::new("Elven spawner"),
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
        Kingdom::Elven,
        Spawner::new(ELVEN_ARCHER),
        SpawnerEventListener,
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
        Spawner::new(GOBLIN_WARRIOR),
        SpawnerTicker::new(0.5),
    ));
    // traps
    commands.spawn((
        Name::new("Elven trap"),
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
        Kingdom::Elven,
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

#[derive(Resource)]
struct StructureAssets {
    spawn_sound: KingdomHandle<AudioSource>,
}

pub struct SpawnerEvent;

#[derive(Component)]
struct Spawner {
    subject_blueprint: SubjectBlueprint,
}

impl Spawner {
    fn new(subject_blueprint: SubjectBlueprint) -> Self {
        Self { subject_blueprint }
    }
}

#[derive(Component)]
struct SpawnerEventListener;

#[derive(Component)]
struct SpawnerTicker {
    stopwatch: Stopwatch,
    average_interval: Duration,
    next_interval: Duration,
}

impl SpawnerTicker {
    fn new(interval_seconds: f32) -> Self {
        Self {
            stopwatch: Stopwatch::new(),
            average_interval: Duration::from_secs_f32(interval_seconds),
            next_interval: Duration::ZERO,
        }
    }
}

#[derive(Component)]
struct Trap;

fn trigger_spawners(
    query: Query<(&Transform, &Kingdom, &Spawner), With<SpawnerEventListener>>,
    mut spawner_events: EventReader<SpawnerEvent>,
    mut spawn_events: EventWriter<SpawnEvent>,
    structure_assets: Res<StructureAssets>,
    audio: Res<Audio>,
) {
    for _ in spawner_events.iter() {
        for (transform, kingdom, spawner) in &query {
            let blueprint = spawner.subject_blueprint.clone();
            let event = SpawnEvent::new(blueprint, transform.translation, *kingdom);
            spawn_events.send(event);

            let sound = structure_assets.spawn_sound.get(*kingdom);
            audio.play(sound).with_volume(0.1);
        }
    }
}

fn tick_spawners(
    mut query: Query<(&Transform, &Kingdom, &Spawner, &mut SpawnerTicker)>,
    mut events: EventWriter<SpawnEvent>,
    time: Res<Time>,
    structure_assets: Res<StructureAssets>,
    audio: Res<Audio>,
) {
    for (transform, kingdom, spawner, mut ticker) in &mut query {
        ticker.stopwatch.tick(time.delta());

        while ticker.stopwatch.elapsed() >= ticker.next_interval {
            let remaining = ticker.stopwatch.elapsed() - ticker.next_interval;
            ticker.stopwatch.set_elapsed(remaining);

            let random_offset = 0.5 + fastrand::f32();
            ticker.next_interval = ticker.average_interval.mul_f32(random_offset);

            let blueprint = spawner.subject_blueprint.clone();
            let event = SpawnEvent::new(blueprint, transform.translation, *kingdom);
            events.send(event);

            let sound = structure_assets.spawn_sound.get(*kingdom);
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
