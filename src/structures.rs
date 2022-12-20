use bevy::{prelude::*, sprite::Anchor};
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    collision::{intersections_with, ColliderBundle},
    subjects::{content::SubjectBlueprint, spawn_subjects, Health, SpawnEvent, Subject},
    Kingdom, KingdomHandle, SKY_HEIGHT, WORLD_EXTENSION,
};

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StructureAssets>()
            .add_event::<NexusSpawnEvent>()
            .add_startup_system(setup)
            .add_system(nexus_spawn_subjects)
            .add_system(check_traps.after(spawn_subjects));
    }
}

fn setup(mut commands: Commands) {
    // spawners
    commands.spawn((
        Name::new("Elven nexus"),
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
        Nexus,
    ));
    commands.spawn((
        Name::new("Monster nexus"),
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
        Nexus,
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

impl FromWorld for StructureAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server: &AssetServer = world.resource();
        StructureAssets {
            spawn_sound: KingdomHandle {
                elven: asset_server.load("sounds/elf_spawn.wav"),
                monster: asset_server.load("sounds/monster_spawn.wav"),
            },
        }
    }
}

pub struct NexusSpawnEvent {
    pub blueprint: &'static SubjectBlueprint,
    pub kingdom: Kingdom,
}

impl NexusSpawnEvent {
    pub fn new(blueprint: &'static SubjectBlueprint, kingdom: Kingdom) -> Self {
        Self { blueprint, kingdom }
    }
}

#[derive(Component)]
struct Nexus;

#[derive(Component)]
struct Trap;

fn nexus_spawn_subjects(
    query: Query<(&Transform, &Kingdom), With<Nexus>>,
    mut nexus_spawn_events: EventReader<NexusSpawnEvent>,
    mut spawn_events: EventWriter<SpawnEvent>,
    structure_assets: Res<StructureAssets>,
    audio: Res<Audio>,
) {
    for nexus_spawn_event in nexus_spawn_events.iter() {
        for (transform, kingdom) in &query {
            if *kingdom != nexus_spawn_event.kingdom {
                continue;
            }

            let blueprint = nexus_spawn_event.blueprint;
            let event = SpawnEvent::new(blueprint, transform.translation, *kingdom);
            spawn_events.send(event);

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
