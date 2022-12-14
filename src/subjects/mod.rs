pub mod content;
pub mod states;
pub mod weapons;

use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;

use crate::{
    animation::{Animation, AnimationMode, AnimationPlayer},
    collision::ColliderBundle,
    Kingdom, PX_PER_METER,
};

use self::{
    content::SubjectBlueprint,
    states::{MovingState, ShootingState, SubjectStatesPlugin, UpdateSubjectState},
    weapons::{Bow, Sword, WeaponsPlugin},
};

pub struct SubjectsPlugin;

impl Plugin for SubjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SubjectStatesPlugin)
            .add_plugin(WeaponsPlugin)
            .init_resource::<Frontlines>()
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

#[derive(Component, Clone)]
pub struct SubjectAnimations {
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
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    commands: &mut Commands,
) {
    let texture = asset_server.load("sprites/archer.png");
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(20.0, 20.0), 9, 5, None, None);
    let animation = &blueprint.animations.moving;

    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlases.add(texture_atlas),
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

    match kingdom {
        Kingdom::Human => {
            root_commands.insert(Bow::new(3.0));
        }
        Kingdom::Monster => {
            root_commands.insert(Sword);
        }
    }

    root_commands.push_children(&[sprite_entity]);
}
