use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;

use crate::{
    collisions::ColliderBundle,
    creatures::{Behaviour, Speed},
    palette,
};

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(spawner);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Human spawner"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.1, 0.1, 0.5),
                custom_size: Some(Vec2::new(0.9, 1.6)),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            transform: Transform::from_xyz(-10.0, 0.0, 0.0),
            ..default()
        },
        Spawner::new(
            "Human",
            palette::LIGHT_PINK,
            Vec2::new(0.7, 1.5),
            1.0,
            Behaviour::MoveRight,
            2.0,
        ),
    ));
    commands.spawn((
        Name::new("Monster spawner"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.1, 0.1, 0.5),
                custom_size: Some(Vec2::new(0.8, 1.0)),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            transform: Transform::from_xyz(10.0, 0.0, 0.0),
            ..default()
        },
        Spawner::new(
            "Monster",
            palette::DARK_BLACK,
            Vec2::new(0.6, 0.9),
            2.0,
            Behaviour::MoveLeft,
            2.0,
        ),
    ));
}

#[derive(Component)]
struct Spawner {
    name: String,
    color: Color,
    size: Vec2,
    speed: f32,
    behaviour: Behaviour,
    timer: Timer,
    spawn_count: u32,
}

impl Spawner {
    fn new(
        name: impl Into<String>,
        color: Color,
        size: Vec2,
        speed: f32,
        behaviour: Behaviour,
        interval_seconds: f32,
    ) -> Self {
        Self {
            name: name.into(),
            size,
            color,
            speed,
            behaviour,
            timer: Timer::from_seconds(interval_seconds, TimerMode::Repeating),
            spawn_count: 0,
        }
    }
}

fn spawner(mut query: Query<(&Transform, &mut Spawner)>, time: Res<Time>, mut commands: Commands) {
    for (transform, mut spawner) in &mut query {
        spawner.timer.tick(time.delta());
        for _ in 0..spawner.timer.times_finished_this_tick() {
            spawner.spawn_count += 1;
            commands.spawn((
                Name::new(format!("{}{}", spawner.name, spawner.spawn_count)),
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
                Speed::new(spawner.speed),
                spawner.behaviour.clone(),
                ColliderBundle::kinematic(Collider::cuboid(
                    spawner.size.x / 2.0,
                    spawner.size.y / 2.0,
                )),
            ));
        }
    }
}
