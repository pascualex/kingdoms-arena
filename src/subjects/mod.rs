pub mod content;
pub mod state;

use bevy::{prelude::*, sprite::Anchor};
use bevy_kira_audio::AudioSource;
use bevy_rapier2d::prelude::*;

use crate::{
    animation::{Animation, AnimationMode, AnimationPlayer},
    collision::ColliderBundle,
    weapons::{content::WeaponsBlueprint, Bow, Sword},
    Kingdom, KingdomHandle, PX_PER_METER,
};

use self::{
    content::SubjectBlueprint,
    state::{MovingState, SubjectStatePlugin, UpdateSubjectState},
};

pub struct SubjectPlugin;

impl Plugin for SubjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SubjectStatePlugin)
            .init_resource::<Frontlines>()
            .add_startup_system(load_assets)
            .add_system(set_subject_velocities.after(UpdateSubjectState))
            .add_system(update_frontlines)
            .add_system(despawn_dead_subjects);
    }
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    let atlas = TextureAtlas::from_grid(
        asset_server.load("sprites/archer.png"),
        Vec2::new(20.0, 19.0),
        7,
        3,
        Some(Vec2::new(0.0, 1.0)),
        Some(Vec2::new(0.0, 1.0)),
    );
    commands.insert_resource(SubjectAssets {
        atlas: atlases.add(atlas),
        death_sound: KingdomHandle {
            human: asset_server.load("sounds/human_death.wav"),
            monster: asset_server.load("sounds/monster_death.wav"),
        },
    });
}

#[derive(Resource)]
pub struct SubjectAssets {
    pub atlas: Handle<TextureAtlas>,
    pub death_sound: KingdomHandle<AudioSource>,
}

#[derive(Resource)]
pub struct Frontlines {
    pub human: Frontline,
    pub monster: Frontline,
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

pub struct Frontline {
    pub position: f32,
    pub entity: Option<Entity>,
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

#[derive(Component, Clone)]
pub struct SubjectAnimations {
    idle: Animation,
    moving: Animation,
    shooting: Animation,
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

pub fn spawn_subject(
    blueprint: &SubjectBlueprint,
    position: Vec3,
    kingdom: Kingdom,
    assets: &SubjectAssets,
    commands: &mut Commands,
) {
    let animation = &blueprint.animations.moving;

    let sprite = SpriteSheetBundle {
        texture_atlas: assets.atlas.clone(),
        sprite: TextureAtlasSprite {
            index: animation.start_index,
            anchor: Anchor::BottomCenter,
            flip_x: matches!(kingdom, Kingdom::Monster),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, -blueprint.size.y / 2.0, 0.0),
            scale: Vec3::splat(1.0 / PX_PER_METER),
            ..default()
        },
        ..default()
    };
    let sprite_entity = commands.spawn(sprite).id();

    let mut root_commands = commands.spawn((
        Name::new(blueprint.name),
        SpatialBundle::from_transform(Transform::from_translation(
            position + Vec3::new(0.0, blueprint.size.y / 2.0, 0.0),
        )),
        AnimationPlayer::new(sprite_entity, animation, AnimationMode::Repeating),
        RigidBody::KinematicVelocityBased,
        ColliderBundle::new(Collider::cuboid(
            blueprint.size.x / 2.0,
            blueprint.size.y / 2.0,
        )),
        Velocity::zero(),
        kingdom,
        Subject,
        Health::new(1),
        Speed(blueprint.speed),
        blueprint.animations.clone(),
        MovingState,
    ));

    match &blueprint.weapon {
        WeaponsBlueprint::Sword => root_commands.insert(Sword),
        WeaponsBlueprint::Bow(b) => root_commands.insert(Bow::new(b.range, b.recharge_seconds)),
    };

    root_commands.push_children(&[sprite_entity]);
}
