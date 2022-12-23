pub mod content;
pub mod state;

use bevy::{ecs::system::SystemState, prelude::*, sprite::Anchor};
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    animation::{Animation, AnimationMode, AnimationPlayer},
    collision::ColliderBundle,
    unit::Health,
    weapon::{content::WeaponKind, Bow, Sword},
    AppState, Kingdom, KingdomHandle, PX_PER_METER,
};

use self::{
    content::SubjectBlueprint,
    state::{MovingState, SubjectStatePlugin, UpdateSubjectState},
};

#[derive(SystemLabel)]
pub struct SpawnSubjects;

#[derive(SystemLabel)]
pub struct DamageSubjects;

pub struct SubjectPlugin;

impl Plugin for SubjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SubjectStatePlugin)
            .init_resource::<SubjectAssets>()
            .add_event::<SpawnEvent>()
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn_subjects))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(spawn_subjects.label(SpawnSubjects))
                    .with_system(set_subject_velocities.after(UpdateSubjectState))
                    .with_system(despawn_dead_subjects.after(DamageSubjects)),
            );
    }
}

#[derive(Resource)]
struct SubjectAssets {
    atlas: Handle<TextureAtlas>,
    death_sound: KingdomHandle<AudioSource>,
}

impl FromWorld for SubjectAssets {
    fn from_world(world: &mut World) -> Self {
        let mut system_state: SystemState<(
            Res<AssetServer>, //
            ResMut<Assets<TextureAtlas>>,
        )> = SystemState::new(world);
        let (asset_server, mut atlases) = system_state.get_mut(world);
        SubjectAssets {
            atlas: atlases.add(TextureAtlas::from_grid(
                asset_server.load("sprites/elven_archer.png"),
                Vec2::splat(20.0),
                7,
                3,
                Some(Vec2::ONE),
                None,
            )),
            death_sound: KingdomHandle {
                elven: asset_server.load("sounds/elf_death.wav"),
                monster: asset_server.load("sounds/monster_death.wav"),
            },
        }
    }
}

pub struct SpawnEvent {
    pub blueprint: &'static SubjectBlueprint,
    pub position: Vec3,
    pub kingdom: Kingdom,
}

impl SpawnEvent {
    pub fn new(blueprint: &'static SubjectBlueprint, position: Vec3, kingdom: Kingdom) -> Self {
        Self {
            blueprint,
            position,
            kingdom,
        }
    }
}

#[derive(Component)]
pub struct Subject;

#[derive(Component, Deref, DerefMut)]
pub struct Speed(pub f32);

#[derive(Component, Clone)]
pub struct SubjectAnimations {
    idle: Animation,
    moving: Animation,
    shooting: Animation,
}

fn spawn_subjects(
    mut events: EventReader<SpawnEvent>,
    assets: Res<SubjectAssets>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let animation = &event.blueprint.animations.moving;

        let sprite = SpriteSheetBundle {
            texture_atlas: assets.atlas.clone(),
            sprite: TextureAtlasSprite {
                index: animation.start_index,
                anchor: Anchor::BottomCenter,
                flip_x: matches!(event.kingdom, Kingdom::Monster),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -event.blueprint.size.y / 2.0, 0.0),
                scale: Vec3::splat(1.0 / PX_PER_METER),
                ..default()
            },
            ..default()
        };
        let sprite_entity = commands.spawn(sprite).id();

        let mut root_commands = commands.spawn((
            Name::new(event.blueprint.name),
            SpatialBundle::from_transform(Transform::from_translation(
                event.position + Vec3::new(0.0, event.blueprint.size.y / 2.0, 0.0),
            )),
            AnimationPlayer::new(sprite_entity, animation, AnimationMode::Repeating),
            RigidBody::KinematicVelocityBased,
            ColliderBundle::new(Collider::cuboid(
                event.blueprint.size.x / 2.0,
                event.blueprint.size.y / 2.0,
            )),
            Velocity::zero(),
            event.kingdom,
            Subject,
            Health::new(event.blueprint.health),
            Speed(event.blueprint.speed),
            event.blueprint.animations.clone(),
            MovingState,
        ));

        match &event.blueprint.weapon.kind {
            WeaponKind::Sword => root_commands.insert(Sword::new(event.blueprint.weapon.damage)),
            WeaponKind::Bow(k) => root_commands.insert(Bow::new(
                event.blueprint.weapon.damage,
                k.range,
                k.spread,
                k.speed,
                k.recharge_seconds,
            )),
        };

        root_commands.push_children(&[sprite_entity]);
    }
}

fn despawn_subjects(query: Query<Entity, With<Subject>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_dead_subjects(
    query: Query<(Entity, &Kingdom, &Health), With<Subject>>,
    subject_assets: Res<SubjectAssets>,
    audio: Res<Audio>,
    mut commands: Commands,
) {
    for (entity, kingdom, health) in &query {
        if health.is_dead() {
            commands.entity(entity).despawn_recursive();

            let sound = subject_assets.death_sound.get(*kingdom);
            audio.play(sound).with_volume(0.2);
        }
    }
}

fn set_subject_velocities(
    mut query: Query<(&mut Velocity, &Kingdom, &Speed, Option<&MovingState>), With<Subject>>,
) {
    for (mut velocity, kingdom, speed, moving_state) in &mut query {
        velocity.linvel.x = match moving_state {
            Some(_) => match kingdom {
                Kingdom::Elven => **speed,
                Kingdom::Monster => -**speed,
            },
            None => 0.0,
        };
    }
}
