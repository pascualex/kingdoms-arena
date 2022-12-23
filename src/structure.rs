use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    collision::ColliderBundle,
    subject::{content::SubjectBlueprint, SpawnEvent, SpawnSubjects},
    unit::Health,
    AppState, Kingdom, KingdomHandle, WORLD_EXTENSION,
};

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StructureAssets>()
            .add_event::<NexusSpawnEvent>()
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_nexuses))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn_nexuses))
            .add_system(spawn_subjects_at_nexuses)
            .add_system(finish_game_on_destroyed_nexus.before(SpawnSubjects));
    }
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

fn spawn_nexuses(mut commands: Commands) {
    commands.spawn((
        Name::new("Elven nexus"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.1, 0.1, 0.5),
                custom_size: Some(Vec2::new(2.0, 3.0)),
                ..default()
            },
            transform: Transform::from_xyz(-WORLD_EXTENSION + 5.0, 1.5, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        ColliderBundle::new(Collider::cuboid(1.0, 1.5)),
        Kingdom::Elven,
        Health::new(50),
        Nexus,
    ));
    commands.spawn((
        Name::new("Monster nexus"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.1, 0.1, 0.5),
                custom_size: Some(Vec2::new(2.0, 3.0)),
                ..default()
            },
            transform: Transform::from_xyz(WORLD_EXTENSION - 5.0, 1.5, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        ColliderBundle::new(Collider::cuboid(1.0, 1.5)),
        Kingdom::Monster,
        Health::new(50),
        Nexus,
    ));
}

fn despawn_nexuses(query: Query<Entity, With<Nexus>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_subjects_at_nexuses(
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

            let mut position = transform.translation;
            position.y = 0.0;

            let blueprint = nexus_spawn_event.blueprint;
            let event = SpawnEvent::new(blueprint, position, *kingdom);
            spawn_events.send(event);

            let sound = structure_assets.spawn_sound.get(*kingdom);
            audio.play(sound).with_volume(0.1);
        }
    }
}

fn finish_game_on_destroyed_nexus(
    query: Query<&Health, With<Nexus>>,
    mut state: ResMut<State<AppState>>,
) {
    for health in &query {
        if health.is_dead() {
            state.set(AppState::Menu).unwrap();
        }
    }
}
